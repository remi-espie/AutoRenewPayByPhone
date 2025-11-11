use crate::check_login::check_login;
use crate::routes::Route;
use crate::types;
use crate::types::AppContext;
use chrono::{Datelike, NaiveTime, Timelike};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
use dioxus::events::FormData;
use dioxus::hooks::{use_context, use_signal};
use dioxus::prelude::Signal;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use dioxus_sdk_storage::use_persistent;

#[component]
pub(crate) fn Park(name: String) -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let context = use_context::<Signal<AppContext>>();
    let api_url = context.read().api_url.clone();
    let mut duration = use_signal(|| "".to_string());
    let mut end_text = use_signal(|| "".to_string());
    let mut danger_text = use_signal(|| "Invalid time".to_string());
    let mut loading_button = use_signal(|| "".to_string());
    let mut disabled_button = use_signal(|| true);
    let mut park_code = use_signal(|| 0);

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
            .post(format!("{}park", context.read().api_url))
            .header("authorization", ["Bearer ", bearer().as_str()].concat())
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
                    park_code.set(500);
                    loading_button.set("".to_string());
                    return;
                }
                info!("Park successful");
                loading_button.set("".to_string());
                park_code.set(res.status().into());
            }
            Err(e) => {
                error!("Park failed: {}", e);
                park_code.set(500);
                loading_button.set("".to_string());
            }
        }
    };

    rsx! {
    div { class: "container is-max-tablet", onmounted: move |_| check_login(bearer(), api_url.clone()),
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

        if park_code() !=0 {
            if park_code() == 202 {
                div { class: "notification is-success mt-3",
                    button { class: "delete", onclick: move |_| park_code.set(0)}
                    p { class:"is-flex is-align-items-center", "Parking successful! {end_text}! You can now" }
                        Link {
                    class: "button is-primary is-fullwidth", to: Route::Home {}, "Go back"}
                }
            }
            else {
                 div { class: "notification is-danger mt-3",
                    button { class: "delete", onclick: move |_| park_code.set(0)}
                    p { class: "is-flex is-align-items-center", "Parking Failed! Please retry later"}
                    }
            }
        }
    }
    }
}
