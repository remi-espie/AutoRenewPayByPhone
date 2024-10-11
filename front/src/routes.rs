use crate::components::home::Home_comp;
use crate::components::login::Login_comp;
use crate::components::park::{Park_comp, Park_compProps};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/home")]
    Home {},
    #[route("/")]
    Login {},
    #[route("/park/:name")]
    Park { name: String },
}

#[component]
fn Home() -> Element {
    Home_comp()
}

#[component]
fn Park(name: String) -> Element {
    Park_comp(Park_compProps { name })
}

#[component]
fn Login() -> Element {
    Login_comp()
}
