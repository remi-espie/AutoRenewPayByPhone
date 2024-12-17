use crate::local_storage::use_persistent;
use crate::routes::Route;
use crate::types::AppContext;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};

#[component]
pub(crate) fn Login() -> Element {
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
                .get(format!("{}healthz", context.read().api_url))
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
        div { class: "container small-container", onvisible: move |_| {
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
