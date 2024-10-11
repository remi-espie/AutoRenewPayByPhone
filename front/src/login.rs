use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_context, use_signal};
use dioxus::prelude::{navigator, spawn, Signal};
use dioxus::signals::Writable;
use dioxus_logger::tracing::{error, info};
use crate::{Route};
use crate::types::AppContext;

#[component]
pub(crate) fn Login() -> Element {
    let nav = navigator();
    let mut context = use_context::<Signal<AppContext>>();
    let mut loading = use_signal(|| false);
    let check = move |_| {
        spawn(async move {
            info!("Bearer token: {}", context().bearer);
            loading.set(true);
            let client = reqwest::Client::new();

            match client
                .get(format!("{}/healthz", context().api_url))
                .header(
                    "authorization",
                    ["Bearer ", context().bearer.as_str()].concat(),
                )
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
                    nav.push(Route::Home {});
                }
                Err(e) => {
                    error!("Login failed: {}", e);
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        h1 {"Login"}
        input { r#type: "text", placeholder: "Bearer token", oninput: move |e| context.write().bearer = e.value(), disabled: loading() }
        button { onclick: check, disabled: loading(), "Login" }
    }
}