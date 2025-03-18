mod check_login;
mod components;
mod env;
mod local_storage;
mod routes;
mod types;

use crate::routes::Route;
use crate::types::AppContext;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

fn main() {
    info!("starting app");
    launch(app);
}

fn app() -> Element {
    info!("using API URL: {}", env::API_URL);
    use_context_provider(|| {
        Signal::new(AppContext {
            api_url: env::API_URL.parse().unwrap(),
        })
    });

    rsx! {
        Router::<Route> {}
    }
}
