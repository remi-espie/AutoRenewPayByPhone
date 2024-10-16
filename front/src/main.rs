mod check_login;
mod components;
mod local_storage;
mod routes;
mod types;

use crate::routes::Route;
use crate::types::AppContext;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::env;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch_web(app);
}

fn app() -> Element {
    use_context_provider(|| {
        Signal::new(AppContext {
            api_url: env::var("API_URL").unwrap_or("https://autopbf.espie.dev/api".to_string()),
        })
    });

    rsx! {
        Router::<Route> {}
    }
}
