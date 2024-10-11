#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

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
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Copy)]
struct AppContext {
    bearer: String,
    client: reqwest::Client,
}

#[component]
fn Login() -> Element {
    let mut context = use_context::<Signal<AppContext>>();
    let mut loading = use_signal(|| false);

    rsx! {
        h1 {"Login"}
        input { r#type: "text", placeholder: "Bearer token", oninput: move |e| context.bearer.set(e.value()), disabled: loading() }
        button {
            onclick: move |_| {
                info!("Bearer token: {}", context.bearer);
                loading.set(true);

        }, disabled: loading(),
            "Login"}
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
