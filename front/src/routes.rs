use crate::components::home::Home;
use crate::components::login::Login;
use crate::components::park::{Park};
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
