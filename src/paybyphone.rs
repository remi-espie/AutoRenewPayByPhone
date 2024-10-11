use regex::Regex;
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct PayByPhone {
    plate: String,
    lot: i32,
    rate: i32,
    login: String,
    password: String,
    payment_account_id: String,
    api_key: Option<String>,
    auth: Option<Auth>,
    account_id: Option<String>,
    client: Client,
}

struct Header {
    user_agent: &'static str,
    accept_language: &'static str,
    accept_encoding: &'static str,
    referer: &'static str,
    origin: &'static str,
    dnt: &'static str,
    connection: &'static str,
}

#[derive(Deserialize, Clone)]
struct Auth {
    token_type: String,
    access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: String,
}

#[derive(Deserialize)]
struct Account {
    id: String,
}

const BASE_HEADERS: Header = Header {
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
    accept_language: "en-US,en;q=0.5",
    accept_encoding: "gzip, deflate",
    referer: "https://paybyphone.com/",
    origin: "https://paybyphone.com",
    dnt: "1",
    connection: "keep-alive",
};

impl PayByPhone {
    pub fn new(
        plate: String,
        lot: i32,
        rate: i32,
        login: String,
        password: String,
        payment_account_id: String,
    ) -> Self {
        Self {
            plate,
            lot,
            rate,
            login,
            password,
            payment_account_id,
            api_key: None,
            auth: None,
            account_id: None,
            client: Client::new(),
        }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("Getting API key...");
        match self
            .client
            .get("https://m2.paybyphone.fr/static/js/main.0aec44c0.chunk.js")
            .send()
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    let pattern = Regex::new(r#"paymentService:\{[^}]*apiKey:"(.*?)""#).unwrap();
                    self.api_key = Some(
                        pattern
                            .captures(&text)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .to_string(),
                    );

                    let params = [
                        ("grant_type", "password"),
                        ("username", &self.login),
                        ("password", &self.password),
                        ("client_id", "paybyphone_webapp"),
                    ];

                    log::info!("Getting user access token...");
                    match self
                        .client
                        .post("https://auth.paybyphoneapis.com/token")
                        .header("User-Agent", BASE_HEADERS.user_agent)
                        .header("Accept-Language", BASE_HEADERS.accept_language)
                        .header("Accept-Encoding", BASE_HEADERS.accept_encoding)
                        .header("Referer", BASE_HEADERS.referer)
                        .header("Origin", BASE_HEADERS.origin)
                        .header("DNT", BASE_HEADERS.dnt)
                        .header("Connection", BASE_HEADERS.connection)
                        .header("Accept", "application/json, text/plain, */*")
                        .header("X-Pbp-ClientType", "WebApp")
                        .form(&params)
                        .send()
                        .await
                    {
                        Ok(resp) => match resp.text().await {
                            Ok(json) => {
                                self.auth = serde_json::from_str(&json).unwrap();
                                log::info!("Getting user account ID...");
                                match self
                                    .get::<String>("https://consumer.paybyphoneapis.com/parking/accounts", None)
                                    .await
                                {
                                    Ok(resp) => match resp.text().await {
                                        Ok(json) => {
                                            let accounts: Vec<Account> =
                                                serde_json::from_str(&json).unwrap();
                                            self.account_id = Some(accounts[0].id.clone());
                                        }
                                        Err(e) => return Err(Box::new(e)),
                                    },
                                    Err(e) => return Err(e),
                                }
                            }
                            Err(e) => return Err(Box::new(e)),
                        },
                        Err(e) => return Err(Box::new(e)),
                    }
                }
                Err(e) => return Err(Box::new(e)),
            },
            Err(e) => return Err(Box::new(e)),
        }
        Ok(())
    }

    pub async fn get<T: Serialize + ?Sized>(&self, url: &str, params: Option<&T>) -> Result<Response, Box<dyn Error>> {
        self.request(Method::GET, url, params).await
    }

    pub async fn request<T: Serialize + ?Sized>(&self, method: Method, url: &str, params: Option<&T>) -> Result<Response, Box<dyn Error>> {
        let mut request = self
            .client
            .request(method.clone(), url)
            .header("User-Agent", BASE_HEADERS.user_agent)
            .header("Accept-Language", BASE_HEADERS.accept_language)
            .header("Accept-Encoding", BASE_HEADERS.accept_encoding)
            .header("Referer", BASE_HEADERS.referer)
            .header("Origin", BASE_HEADERS.origin)
            .header("DNT", BASE_HEADERS.dnt)
            .header("Connection", BASE_HEADERS.connection)
            .header("Accept", "application/json, text/plain, */*")
            .header("x-pbp-version", "2")
            .header("x-api-key", self.api_key.clone().unwrap())
            .header(
                "authorization",
                [
                    self.auth.clone().unwrap().token_type,
                    " ".parse().unwrap(),
                    self.auth.clone().unwrap().access_token,
                ]
                    .concat(),
            );

        if let Some(params) = params {
            match method {
                Method::GET => {
                    request = request.query(params);
                }
                Method::POST => {
                    request = request.form(params);
                }
                _ => {
                }
            }
        }

        match request
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_vehicles(&self) -> Result<String, Box<dyn Error>> {
        log::info!("Getting user vehicles...");
        match self.get::<String>("https://consumer.paybyphoneapis.com/identity/profileservice/v1/members/vehicles/paybyphone", None).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(json) => {
                        Ok(json)
                    }
                    Err(e) => Err(Box::new(e))
                }
            }
            Err(e) => Err(e)
        }
    }

    pub(crate) async fn park(&self) {
        todo!()
    }

    pub(crate) async fn renew(&self) {
        todo!()
    }

    async fn get_parking_session(&self) -> Result<String, Box<dyn Error>> {
        match self.get(format!("https://consumer.paybyphoneapis.com/parking/accounts/{}/sessions", self.account_id.clone().unwrap()).as_str(), Some(&[("periodType","Current")])).await { 
            Ok(resp) => {
                match resp.text().await {
                    Ok(json) => {
                        Ok(json)
                    }
                    Err(e) => {
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    
    pub(crate) async fn check(&self) -> Result<String, Box<dyn Error>> {
        log::info!("Checking user parking sessions...");
        match self.get_parking_session().await {
            Ok(json) => {
                let session = serde_json::from_str::<serde_json::Value>(&json).unwrap();
                let mut sessions = String::new();
                for s in session.as_array().unwrap() {
                    sessions.push_str(&format!("{}\n", s));
                }
                Ok(sessions)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub(crate) async fn cancel(&self) {
        todo!()
    }
}
