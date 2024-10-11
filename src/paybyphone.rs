use crate::types::{
    Account, Auth, Duration, GetParkingSession, GetQuote, GetRateOptions, ParkingOption,
    ParkingSession, PaymentMethod, PaymentPayload, PostQuote, Quote, Vehicle,
};
use regex::Regex;
use reqwest::{Client, Method, Response};
use serde::Serialize;
use serde_json::json;
use std::error::Error;

pub struct PayByPhone {
    plate: String,
    lot: i32,
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
        login: String,
        password: String,
        payment_account_id: String,
    ) -> Self {
        Self {
            plate,
            lot,
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
                                    .get::<String>(
                                        "https://consumer.paybyphoneapis.com/parking/accounts",
                                        None,
                                    )
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

    async fn get<T: Serialize + ?Sized>(
        &self,
        url: &str,
        params: Option<&T>,
    ) -> Result<Response, Box<dyn Error>> {
        self.request(Method::GET, url, params).await
    }

    async fn post<T: Serialize + ?Sized>(
        &self,
        url: &str,
        params: Option<&T>,
    ) -> Result<Response, Box<dyn Error>> {
        self.request(Method::POST, url, params).await
    }

    pub async fn request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        url: &str,
        params: Option<&T>,
    ) -> Result<Response, Box<dyn Error>> {
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
                    log::debug!("Request: {:?}", json!(params));
                    request = request.json(&json!(params));
                }
                _ => {}
            }
        }
        log::debug!("Request: {:?}", request);

        match request.send().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_vehicles(&self) -> Result<Vec<Vehicle>, Box<dyn Error>> {
        log::info!("Getting user vehicles...");
        match self.get::<String>("https://consumer.paybyphoneapis.com/identity/profileservice/v1/members/vehicles/paybyphone", None).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(json) => {
                        match serde_json::from_str::<Vec<Vehicle>>(&json) {
                            Ok(vehicles) => {
                                Ok(vehicles)
                            }
                            Err(e) => {
                                Err(Box::new(e))
                            }
                        }
                    }
                    Err(e) => Err(Box::new(e))
                }
            }
            Err(e) => Err(e)
        }
    }

    pub(crate) async fn park(&self, duration: i32) -> Result<ParkingSession, Box<dyn Error>> {
        // match self.check().await {
        //     Ok(session) => {
        //         log::info!("User already parked");
        //         Ok(session)
        //     }
        //     Err(_) => {
        log::info!("Parking user...");
        match self.get_rate_option().await {
            Ok(options) => {
                log::info!("Got rate options");
                let rate = options[0].clone().rate_option_id;
                match self.get_quote(duration, &rate).await {
                    Ok(quote) => match self.post_quote(quote, duration, &rate).await {
                        Ok(session) => Ok(session),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }

        // }
        // }
    }

    async fn get_quote(&self, duration: i32, rate: &str) -> Result<Quote, Box<dyn Error>> {
        log::info!("Getting quote...");
        match self
            .get(
                format!(
                    "https://consumer.paybyphoneapis.com/parking/accounts/{}/quote",
                    self.account_id.clone().unwrap()
                )
                .as_str(),
                Some(&GetQuote {
                    location_id: self.lot.to_string(),
                    rate_option_id: rate.to_string(),
                    duration_quantity: duration,
                    time_unit: "Minutes".to_string(),
                    license_plate: self.plate.clone(),
                }),
            )
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(json) => match serde_json::from_str::<Quote>(&json) {
                    Ok(quote) => Ok(quote),
                    Err(e) => Err(Box::new(e)),
                },
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(e),
        }
    }

    async fn post_quote(
        &self,
        quote: Quote,
        duration: i32,
        rate: &str,
    ) -> Result<ParkingSession, Box<dyn Error>> {
        log::info!("Post quote...");
        match self
            .post(
                format!(
                    "https://consumer.paybyphoneapis.com/parking/accounts/{}/sessions/",
                    self.account_id.clone().unwrap()
                )
                .as_str(),
                Some(&PostQuote {
                    license_plate: self.plate.clone(),
                    location_id: self.lot.to_string(),
                    stall: None,
                    rate_option_id: rate.to_string(),
                    start_time: quote.parking_start_time,
                    quote_id: quote.quote_id,
                    duration: Duration {
                        quantity: duration,
                        time_unit: "Minutes".to_string(),
                    },
                    payment_method: PaymentMethod {
                        payment_method_type: "PaymentAccount".to_string(),
                        payload: PaymentPayload {
                            payment_account_id: self.payment_account_id.clone(),
                            cvv: None,
                        },
                    },
                }),
            )
            .await
        {
            Ok(resp) => match resp.status() {
                reqwest::StatusCode::ACCEPTED => {
                    log::info!("Parking successful");
                    match self.check().await {
                        Ok(session) => Ok(session),
                        Err(e) => Err(e),
                    }
                }
                _ => Err(format!("Failed to park: {:?}", resp.text().await).into()),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn renew(&self) {
        todo!()
    }

    async fn get_parking_session(&self) -> Result<Vec<ParkingSession>, Box<dyn Error>> {
        match self
            .get(
                format!(
                    "https://consumer.paybyphoneapis.com/parking/accounts/{}/sessions",
                    self.account_id.clone().unwrap()
                )
                .as_str(),
                Some(&GetParkingSession {
                    period_type: "Current".to_string(),
                }),
            )
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(json) => match serde_json::from_str::<Vec<ParkingSession>>(&json) {
                    Ok(sessions) => Ok(sessions),
                    Err(e) => Err(Box::new(e)),
                },
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn check(&self) -> Result<ParkingSession, Box<dyn Error>> {
        log::info!("Checking user parking sessions...");
        match self.get_parking_session().await {
            Ok(session) => {
                match session
                    .iter()
                    .find(|s| s.vehicle.license_plate == self.plate)
                {
                    Some(s) => Ok(s.clone()),
                    None => Err(Box::from("No active parking session found".to_string())),
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn get_rate_option(&self) -> Result<Vec<ParkingOption>, Box<dyn Error>> {
        match self
            .get(
                format!(
                    "https://consumer.paybyphoneapis.com/parking/locations/{}/rateOptions",
                    self.lot
                )
                .as_str(),
                Some(&GetRateOptions {
                    location_id: self.lot.to_string(),
                    license_plate: self.plate.clone(),
                }),
            )
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(json) => match serde_json::from_str::<Vec<ParkingOption>>(&json) {
                    Ok(options) => Ok(options),
                    Err(e) => Err(Box::new(e)),
                },
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn cancel(&self) {
        todo!()
    }
}
