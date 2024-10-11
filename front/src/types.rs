use serde::{Deserialize, Serialize};

pub(crate) struct AppContext {
    pub(crate) api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Accounts {
    pub(crate) accounts: Vec<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) plate: String,
    pub(crate) lot: i32,
}
