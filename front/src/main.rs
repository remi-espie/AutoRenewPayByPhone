mod local_storage;
mod types;

use crate::local_storage::use_persistent;
use crate::types::AppContext;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info, Level};
use dotenvy::dotenv;

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
            api_url: dotenvy::var("API_URL").unwrap_or("http://localhost:3000".to_string()),
        })
    });

    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub(crate) fn Home() -> Element {
    let bearer = use_persistent("bearer", || "".to_string());
    let accounts = use_resource(move || async move {
        let context = use_context::<Signal<AppContext>>();
        let client = reqwest::Client::new();
        match client
            .get(format!("{}/accounts", context.read().api_url))
            .header("authorization", ["Bearer ", bearer.get().as_str()].concat())
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(accounts) => match serde_json::from_str::<types::Accounts>(&accounts) {
                    Ok(accounts) => accounts.accounts,
                    Err(e) => {
                        error!("Failed to parse accounts: {}", e);
                        vec![]
                    }
                },
                Err(e) => {
                    error!("Failed to parse accounts: {}", e);
                    vec![]
                }
            },
            Err(e) => {
                error!("Failed to fetch accounts: {}", e);
                vec![]
            }
        }
    });

    rsx! {
        div { class: "container is-max-tablet", onmounted: move |_| check_login(),
            h1 { class: "is-size-1 has-text-centered", "Accounts" }
            for config in accounts.read().iter() {
                            for account in config.iter() {
                    div { class: "card",
                        div { class: "card-content",
                            div { class: "media",
                                div { class: "media-left",
                                    figure { class: "image is-48x48",
                                        img { src: "https://bulma.io/assets/images/placeholders/96x96.png" }
                                    }
                                }
                                div { class: "media-content is-flex is-align-self-center",
                                    p { class: "title is-4", "{account.plate.clone()}" }
                                    }
                                }
                            div { class: "content",
                                p { class: "title is-4", "{account.name.clone()}" }
                                p { class: "subtitle", "Lot n. {account.lot.to_string()}" }
                            }
                            footer { class: "card-footer",
                                a { class: "card-footer-item button is-primary", href: format!("/park/{}", account.name.clone()), "Park" }
                            }
                        }
                    }
                }
            }
        }
    }
}

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
                .get(format!("{}/healthz", context.read().api_url))
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
        div { class: "container small-container", onmounted: move |_| {
            if !bearer.get().is_empty() {
                info!("Bearer token found, checking...");
                login();
            }
        },
        h1 { class: "is-size-1 has-text-centered", "Login" }
        form { onsubmit: move |_| login(),
            div { class: "field",
                label { class: "label", "Bearer token" }
            input { name: "Bearer token", class: "input", placeholder: "Bearer token", oninput: move |e| bearer.set(e.value()), disabled: loading(), value: bearer.get() }
                }
            input { r#type: "submit", disabled: loading(), class:"button is-primary is-fullwidth", "Login" }
        }
    }
        }
}

async fn check_login() {
    let bearer = use_persistent("bearer", || "".to_string());
    let nav = navigator();
    let context = use_context::<Signal<AppContext>>();
    info!("Bearer token: {}", bearer.get());
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
