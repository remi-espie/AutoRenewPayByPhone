mod check_login;
mod components;
mod local_storage;
mod routes;
mod types;

use crate::routes::Route;
use crate::types::AppContext;
use clap::Parser;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Rémi Espié", about, long_about = None)]
struct Args {
    /// The API URL to use. Default is https://autopbf.espie.dev/api.
    #[arg(short, long, default_value = "https://autopbf.espie.dev/api")]
    api_url: String,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app);
}

fn app() -> Element {
    let args = Args::parse();
    info!("using API URL: {}", args.api_url);
    use_context_provider(|| {
        Signal::new(AppContext {
            api_url: args.api_url,
        })
    });

    rsx! {
        Router::<Route> {}
    }
}
