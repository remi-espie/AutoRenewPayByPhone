mod local_storage;
mod types;

use chrono_tz::Europe::Paris;
use crate::local_storage::use_persistent;
use crate::types::{Accounts, AppContext, Config, ParkingSession};
use chrono::{Datelike, NaiveTime, Timelike, DateTime};
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info, Level};
use dotenvy::dotenv;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/home")]
    Home {},
    #[route("/")]
    Login {},
    #[route("/park/:name")]
    Park { name: String },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dotenv().ok();
    info!("starting app");
    launch_web(app);
}

fn app() -> Element {
    use_context_provider(|| {
        Signal::new(AppContext {
            api_url: dotenvy::var("API_URL").unwrap_or("http://localhost:3000".to_string()),
        })
    });

    rsx! {
        Router::<Route> {}
    }
}

async fn check_login() {
    let bearer = use_persistent("bearer", || "".to_string());
    info!("Bearer token: {}", bearer.get());
    let nav = navigator();
    let context = use_context::<Signal<AppContext>>();
    let client = reqwest::Client::new();

    match client
        .get(format!("{}/healthz", context.read().api_url))
        .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
        .send()
        .await
    {
        Ok(res) => {
            if !res.status().is_success() {
                error!("Login failed: {}", res.text().await.unwrap());
                nav.replace(Route::Login {});
                return;
            }
            info!("Login successful");
        }
        Err(e) => {
            error!("Login failed: {}", e);
            nav.replace(Route::Login {});
        }
    }
}

#[component]
fn Login() -> Element {
    let mut bearer = use_persistent("bearer", || "".to_string());
    let nav = navigator();
    let context = use_context::<Signal<AppContext>>();
    let mut loading = use_signal(|| false);
    let login = move || {
        spawn(async move {
            info!("Bearer token: {}", bearer.get());
            loading.set(true);
            let client = reqwest::Client::new();

            match client
                .get(format!("{}/healthz", context.read().api_url))
                .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
                .send()
                .await
            {
                Ok(res) => {
                    if !res.status().is_success() {
                        error!("Login failed: {}", res.text().await.unwrap());
                        loading.set(false);
                        return;
                    }
                    info!("Login successful");
                    loading.set(false);
                    nav.replace(Route::Home {});
                }
                Err(e) => {
                    error!("Login failed: {}", e);
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "container small-container", onmounted: move |_| {
            if !bearer.get().is_empty() {
                info!("Bearer token found, checking...");
                login();
            }
        },
        h1 { class: "is-size-1 has-text-centered", "Login" }
        form { onsubmit: move |_| login(),
            div { class: "field",
                label { class: "label", "Bearer token" }
                input { name: "Bearer token", class: "input", placeholder: "Bearer token", required: true, oninput: move |e| bearer.set(e.value()), disabled: loading(), value: bearer.get() }
                }
            input { r#type: "submit", disabled: loading(), class:"button is-primary is-fullwidth", "Login" }
            }
        }
    }
}

#[component]
fn AccountCard(account: Config) -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let acc = account.clone();
    let mut session = use_signal(ParkingSession::default);
    let mut loading_session = use_signal(|| true);
    let mut start_time = use_signal(|| "".to_string());
    let mut expiry_time = use_signal(|| "".to_string());

    let _ = use_resource(move || {
        let account = acc.clone();
        async move {
            {
                info!("Checking account: {}", account.name);
                let context = use_context::<Signal<AppContext>>();
                let client = reqwest::Client::new();

                match client
                    .get(format!("{}/check?name={}", context.read().api_url, account.name))
                    .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
                    .send()
                    .await
                {
                    Ok(res) => match res.text().await {
                        Ok(json) => match serde_json::from_str::<ParkingSession>(&json) {
                            Ok(sess) => {
                                match DateTime::parse_from_rfc3339(sess.start_time.as_str()) {
                                    Ok(time) => {
                                        let local_time = time.with_timezone(&Paris);
                                        start_time.set(local_time.format("%H:%M").to_string()); }
                                    Err(e) => {
                                        error!("Failed to parse start time: {}", e);
                                        return;
                                    }
                                }
                                match DateTime::parse_from_rfc3339(sess.expire_time.as_str()) {
                                    Ok(time) => {
                                        let local_time = time.with_timezone(&Paris);
                                        expiry_time.set(local_time.format("%H:%M").to_string()); }
                                    Err(e) => {
                                        error!("Failed to parse end time: {}", e);
                                        return;
                                    }
                                }
                                session.set(sess);
                            }
                            Err(e) => {
                                error!("Failed to parse parking session: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Can't check account {}: {}", account.name, e);
                        }
                    },
                    Err(e) => {
                        error!("Can't check account {}: {}", account.name, e);
                    }
                };
                loading_session.set(false);
            }
        }
    });

    rsx! {
    div { class: "card",
        div { class: "card-content",
                div { class: "content is-flex is-justify-content-space-between",
                    div {
            div { class: "media",
                div { class: "media-left",
                    figure { class: "image is-48x48",
                        img { src: "https://bulma.io/assets/images/placeholders/96x96.png" }
                        }
                    }
                div { class: "media-content is-flex is-align-self-center",
                    p { class: "title is-4", "{account.plate.clone()}" }
                    }
                }
                    div {
                    p { class: "title is-4", "{account.name.clone()}" }
                    p { class: "subtitle", "Lot n. {account.lot.to_string()}" }
                        }
                    }
                    div {
                        if loading_session() {
                            div {class: "skeleton-block"}
                        } else {
                            if session().expire_time.is_empty() {
                                p { class: "title is-4", "Not parked" }
                            } else {
                                p { class: "title is-4 is-spaced", "Session" }
                                p { class: "subtitle", "Start: {start_time}" }
                                p { class: "subtitle has-text-danger", "End: {expiry_time}" }
                        }
                        }
                    }
                }
                footer { class: "card-footer",
                    Link {
                        class: if loading_session() {"card-footer-item button is-primary is-loading"} else if !session().expire_time.is_empty() {"card-footer-item button is-primary is-static"} else {"card-footer-item button is-primary"},
                        to: Route::Park {
                            name: account.name.to_string()
                        },
                        "Park"
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let mut accounts = use_signal(Vec::<Config>::new);
    let _ = use_resource(move || async move {
        let context = use_context::<Signal<AppContext>>();
        let client = reqwest::Client::new();
        match client
            .get(format!("{}/accounts", context.read().api_url))
            .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(json) => match serde_json::from_str::<Accounts>(&json) {
                    Ok(acc) => {
                        accounts.set(acc.accounts);
                    }
                    Err(e) => {
                        error!("Failed to parse accounts: {}", e);
                    }
                },
                Err(e) => {
                    error!("Failed to parse accounts: {}", e);
                }
            },
            Err(e) => {
                error!("Failed to fetch accounts: {}", e);
            }
        }
    });

    rsx! {
        div { class: "container is-max-tablet", onmounted: move |_| check_login(),
            h1 { class: "is-size-1 has-text-centered", "Accounts" }
            for account in accounts() {
                AccountCard { account: account }
            }
        }
    }
}

#[component]
fn Park(name: String) -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let context = use_context::<Signal<AppContext>>();
    let mut duration = use_signal(|| "".to_string());
    let mut end_text = use_signal(|| "".to_string());
    let mut danger_text = use_signal(|| "Invalid time".to_string());
    let mut loading_button = use_signal(|| "".to_string());
    let mut disabled_button = use_signal(|| true);

    let check_time = move |e: Event<FormData>| {
        duration.set(e.value());
        let dur = match NaiveTime::parse_from_str(e.value().as_str(), "%H:%M") {
            Ok(dura) => dura,
            Err(_) => {
                disabled_button.set(true);
                danger_text.set("Invalid time".to_string());
                end_text.set("".to_string());
                return;
            }
        };
        let time_final = chrono::Local::now()
            + chrono::Duration::minutes(dur.minute() as i64)
            + chrono::Duration::hours(dur.hour() as i64);
        if time_final < chrono::Local::now() {
            disabled_button.set(true);
            danger_text.set("Invalid time".to_string());
            end_text.set("".to_string());
            return;
        }
        disabled_button.set(false);
        danger_text.set("".to_string());
        if time_final.num_days_from_ce() > chrono::Local::now().num_days_from_ce() {
            let formated_date = time_final.format("%d/%m/%Y").to_string();
            let formated_time = time_final.format("%H:%M").to_string();
            end_text.set(format!(
                "This session will end at the soonest the {} at {}",
                formated_date, formated_time
            ));
        } else {
            end_text.set(format!(
                "This session will end at the soonest at {}",
                time_final.format("%H:%M")
            ));
        }
    };

    let post_park = move |name: String| async move {
        loading_button.set("is-loading".to_string());
        let naive_dur = NaiveTime::parse_from_str(duration().as_str(), "%H:%M").unwrap();
        let dur = naive_dur.minute() as i16 + naive_dur.hour() as i16 * 60;

        let client = reqwest::Client::new();
        match client
            .post(format!("{}/park", context.read().api_url))
            .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
            .json(&types::Parking {
                name: name.clone(),
                duration: dur,
            })
            .send()
            .await
        {
            Ok(res) => {
                if !res.status().is_success() {
                    error!("Park failed: {}", res.text().await.unwrap());
                    loading_button.set("".to_string());
                    return;
                }
                info!("Park successful");
                loading_button.set("".to_string());
            }
            Err(e) => {
                error!("Park failed: {}", e);
                loading_button.set("".to_string());
            }
        }
    };

    rsx! {
    div { class: "container is-max-tablet", onmounted: move |_| check_login(),
        h1 { class: "is-size-1 has-text-centered", "Park ",
                span { class: "has-text-weight-bold has-text-primary", "{name}"}
            }
        form { onsubmit: move |_| post_park(name.clone()),
            div { class: "field",
                label { class: "label", "Duration" }
                input { r#type: "time", class: "input", placeholder: "Time", required: true, name: "time", oninput: check_time }
                p { class: "help", "{end_text}" }
                p { class: "help is-danger", "{danger_text}" }
                }
             button { disabled: disabled_button(), class:"button is-primary is-fullwidth {loading_button}", "Park" }
            }
        }
    }
}
