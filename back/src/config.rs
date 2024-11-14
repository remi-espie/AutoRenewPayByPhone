use serde::{Deserialize, Serialize};
use std::fs;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Accounts {
    pub(crate) accounts: Vec<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) plate: String,
    pub(crate) lot: i32,
    #[serde(skip_serializing)]
    pub(crate) pay_by_phone: PayByPhone,
    #[serde(skip)]
    pub(crate) session: Option<Session>
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Session {
    pub(crate) next_check: DateTime<Utc>,
    pub(crate) duration: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PayByPhone {
    pub(crate) login: String,
    pub(crate) password: String,
    pub(crate) payment_account_id: String,
}

pub(crate) fn read(file_path: &str) -> Result<Accounts, Box<dyn std::error::Error>> {
    match fs::read_to_string(file_path) {
        Ok(contents) => match serde_yaml::from_str(&contents) {
            Ok(accounts) => Ok(accounts),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}
