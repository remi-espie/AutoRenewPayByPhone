use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Accounts {
    accounts: Vec<Config>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    name: String,
    plate: String,
    lot: i32,
    rate: i32,
    pay_by_phone: PayByPhone,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PayByPhone {
    login: String,
    password: String,
    payment_account_id: String,
}

pub(crate) fn read(file_path: &str) -> Result<Accounts, Box<dyn std::error::Error>> {
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            match serde_yaml::from_str(&contents) {
                Ok(accounts) => Ok(accounts),
                Err(e) => Err(Box::new(e))
            }
        },
        Err(e) => {
            Err(Box::new(e))
        }
    }
}