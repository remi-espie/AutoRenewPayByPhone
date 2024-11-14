use crate::local_storage::use_persistent;
use crate::routes::Route;
use crate::types::{AppContext, Config, ParkingSession, RenewSession};
use chrono::{DateTime, Datelike};
use chrono_tz::Europe::Paris;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};

#[component]
pub(crate) fn AccountCard_comp(account: Config) -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let acc = account.clone();
    let mut session = use_signal(ParkingSession::default);
    let mut loading_session = use_signal(|| true);
    let mut renew_time = use_signal(|| "".to_string());
    let mut renew_duration = use_signal(|| "".to_string());
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
                    .get(format!(
                        "{}/check?name={}",
                        context.read().api_url,
                        account.name
                    ))
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
                                        start_time.set(local_time.format("%H:%M").to_string());
                                    }
                                    Err(e) => {
                                        error!("Failed to parse start time: {}", e);
                                        return;
                                    }
                                }
                                match DateTime::parse_from_rfc3339(sess.expire_time.as_str()) {
                                    Ok(time) => {
                                        let local_time = time.with_timezone(&Paris);
                                        if local_time.num_days_from_ce()
                                            > chrono::Local::now().num_days_from_ce()
                                        {
                                            expiry_time.set(
                                                local_time.format("%d/%m/%Y - %H:%M").to_string(),
                                            );
                                        } else {
                                            expiry_time.set(local_time.format("%H:%M").to_string());
                                        }
                                    }
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
                match client
                    .get(format!(
                        "{}/check_renew?name={}",
                        context.read().api_url,
                        account.name
                    ))
                    .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
                    .send()
                    .await
                {
                    Ok(res) => match res.text().await {
                        Ok(json) => match serde_json::from_str::<RenewSession>(&json) {
                            Ok(sess) => {
                                match DateTime::parse_from_rfc3339(sess.next_check.as_str()) {
                                    Ok(time) => {
                                        let local_time = time.with_timezone(&Paris);
                                        renew_time.set(local_time.format("%H:%M").to_string());
                                    }
                                    Err(e) => {
                                        error!("Failed to parse start time: {}", e);
                                        return;
                                    }
                                }
                                renew_duration.set(sess.duration.to_string());
                            }
                            Err(e) => {
                                error!("Failed to parse renew session: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Can't check renew account {}: {}", account.name, e);
                        }
                    },
                    Err(e) => {
                        error!("Can't check renew account {}: {}", account.name, e);
                    }
                }
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
                                p { class: "subtitle", "Next renew: {renew_time}" }
                                p { class: "subtitle", "For at least: {renew_duration}min" }
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
