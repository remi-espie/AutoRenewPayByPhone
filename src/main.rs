mod config;
mod paybyphone;
mod types;

use clap::Parser;
use std::error::Error;

#[derive(Debug, Clone)]
enum Action {
    Park,
    Renew,
    Check,
    Cancel,
    Vehicles,
}

impl From<String> for Action {
    fn from(other: String) -> Self {
        match other.as_str() {
            "park" => Action::Park,
            "renew" => Action::Renew,
            "check" => Action::Check,
            "cancel" => Action::Cancel,
            "vehicles" => Action::Vehicles,
            _ => panic!("Invalid action"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Rémi Espié", about, long_about = None)]
struct Args {
    /// Action to perform
    #[arg(short, long)]
    action: Action,

    /// Account name from config.yaml
    #[arg(short = 'x', long)]
    account: String,

    /// Duration in minutes
    #[arg(short, long)]
    duration: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let args = Args::parse();

    log::info!("Reading user config...");
    let config = config::read("config.yaml")?;
    log::info!("Initializing PayByPhone...");
    let account = config
        .accounts
        .iter()
        .find(|a| a.name == args.account)
        .unwrap_or_else(|| panic!("Account not found"));
    let mut pay_by_phone = paybyphone::PayByPhone::new(
        account.plate.clone(),
        account.lot,
        account.rate,
        account.pay_by_phone.login.clone(),
        account.pay_by_phone.password.clone(),
        account.pay_by_phone.payment_account_id.clone(),
    );
    match pay_by_phone.init().await {
        Ok(_) => {
            log::info!("PayByPhone initialized");
        }
        Err(e) => {
            log::error!("{:?}", e);
            panic!("Failed to initialize PayByPhone");
        }
    }

    match args.action {
        Action::Park => {
            log::info!("Parking...");
            match args.duration {
                Some(duration) => {
                    println!("{:?}", pay_by_phone.park(duration).await);
                }
                None => {
                    panic!("Duration is required for park action");
                }
            }
        }
        Action::Renew => {
            log::info!("Renewing...");
            println!("{:?}", pay_by_phone.renew().await);
        }
        Action::Check => {
            log::info!("Checking...");
            match pay_by_phone.check().await {
                Ok(sessions) => {
                    println!("{:?}", sessions);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        }
        Action::Cancel => {
            log::info!("Cancelling...");
            println!("{:?}", pay_by_phone.cancel().await);
        }
        Action::Vehicles => {
            log::info!("Getting vehicles...");
            match pay_by_phone.get_vehicles().await {
                Ok(vehicles) => {
                    println!("{:?}", vehicles);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        }
    }
    Ok(())
}
