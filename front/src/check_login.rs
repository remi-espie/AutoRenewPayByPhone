use crate::local_storage::use_persistent;
use crate::routes::Route;
use crate::types::AppContext;
use dioxus::hooks::use_context;
use dioxus::prelude::{navigator, Readable, Signal};
use dioxus_logger::tracing::{error, info};

pub(crate) async fn check_login() {
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
