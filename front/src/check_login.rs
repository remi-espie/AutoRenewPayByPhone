use crate::local_storage::use_persistent;
use crate::routes::Route;
use crate::types::AppContext;
use dioxus::hooks::use_context;
use dioxus::prelude::{navigator, Readable, Signal};
use dioxus_logger::tracing::{error, info};

pub(crate) async fn check_login(bearer: String, api_url: String) {
    info!("Bearer token: {}", bearer);
    let nav = navigator();
    let client = reqwest::Client::new();

    match client
        .get(format!("{}healthz", api_url))
        .header("authorization", ["Bearer ", bearer.as_str()].concat())
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
