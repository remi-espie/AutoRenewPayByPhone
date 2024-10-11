mod login;
mod types;
mod home;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dotenvy::dotenv;
use crate::types::AppContext;
use crate::login::Login;
use crate::home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/home")]
    Home {},
    #[route("/")]
    Login {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dotenv().ok();
    info!("starting app");
    launch(app);
}

fn app() -> Element {
    use_context_provider(|| {
        Signal::new(AppContext {
            bearer: "".to_string(),
            api_url: dotenvy::var("API_URL").unwrap_or("http://localhost:3000".to_string()),
        })
    });

    rsx! {
        Router::<Route> {}
    }
}
