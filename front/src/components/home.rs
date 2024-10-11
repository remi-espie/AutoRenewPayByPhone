use crate::check_login::check_login;
use crate::components::account_card::AccountCard_comp;
use crate::local_storage::use_persistent;
use crate::types::{Accounts, AppContext, Config};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_context, use_resource, use_signal};
use dioxus::prelude::Signal;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;

#[component]
pub(crate) fn Home_comp() -> Element {
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
                AccountCard_comp { account: account }
            }
        }
    }
}
