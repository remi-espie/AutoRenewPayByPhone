mod config;
mod paybyphone;
mod types;
mod middleware;

use clap::Parser;
use std::error::Error;
use std::fmt::format;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::State;
use axum::handler::Handler;
use axum::middleware::from_fn;
use dotenvy::dotenv;
use crate::config::Accounts;
use crate::middleware::auth_middleware;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Rémi Espié", about, long_about = None)]
struct Args {
    /// The port the application will listen on. Default is 3000.
    #[arg(short, long, default_value = "3000")]
    port: u16,
    
    /// Bearer token for authentication. Can be set through the BEARER environment variable.
    #[arg(short, long, env)]
    bearer: String,
    
    // /// Action to perform
    // #[arg(short, long)]
    // action: Action,
    // 
    // /// Account name from config.yaml
    // #[arg(short = 'x', long)]
    // account: String,
    // 
    // /// Duration in minutes
    // #[arg(short, long)]
    // duration: Option<i32>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let args = Args::parse();
    let bearer_token = Arc::new(args.bearer.clone());

    log::info!("Reading user config...");
    let config = config::read("config.yaml").unwrap_or_else(|e| panic!("{:?}", e));
    
    let app = Router::new()
        .route("/healthz", get(()))
        .route("/accounts", get(get_accounts))
        .route("/park", post(()))
        .route("/check", get(check))
        .route("/vehicles", get(()))
        .with_state(config)
        .layer(from_fn(move |req, next| auth_middleware(req, next, bearer_token.clone())));
    // .route("/cancel", post(|| async { Json(StatusCode::OK) }))
    // .route("/renew", post(|| async { Json(StatusCode::OK) }))


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await.unwrap();
    log::info!("Listening on 0.0.0.0:{}", args.port);
    axum::serve(listener, app).await.unwrap();
    
}

async fn check() -> StatusCode {
    StatusCode::OK
}

async fn get_accounts(State(config): State<Accounts>) -> Json<config::Accounts> {
    Json(config)
}

// log::info!("Initializing PayByPhone...");
// let account = config
//     .accounts
//     .iter()
//     .find(|a| a.name == args.account)
//     .unwrap_or_else(|| panic!("Account not found"));
// let mut pay_by_phone = paybyphone::PayByPhone::new(
//     account.plate.clone(),
//     account.lot,
//     account.pay_by_phone.login.clone(),
//     account.pay_by_phone.password.clone(),
//     account.pay_by_phone.payment_account_id.clone(),
// );
// match pay_by_phone.init().await {
//     Ok(_) => {
//         log::info!("PayByPhone initialized");
//     }
//     Err(e) => {
//         log::error!("{:?}", e);
//         panic!("Failed to initialize PayByPhone");
//     }
// }
// 
// match args.action {
//     Action::Park => {
//         log::info!("Parking...");
//         match args.duration {
//             Some(duration) => {
//                 println!("{:?}", pay_by_phone.park(duration).await);
//             }
//             None => {
//                 panic!("Duration is required for park action");
//             }
//         }
//     }
//     Action::Renew => {
//         log::info!("Renewing...");
//         println!("{:?}", pay_by_phone.renew().await);
//     }
//     Action::Check => {
//         log::info!("Checking...");
//         match pay_by_phone.check().await {
//             Ok(sessions) => {
//                 println!("{:?}", sessions);
//             }
//             Err(e) => {
//                 log::error!("{:?}", e);
//             }
//         }
//     }
//     Action::Cancel => {
//         log::info!("Cancelling...");
//         println!("{:?}", pay_by_phone.cancel().await);
//     }
//     Action::Vehicles => {
//         log::info!("Getting vehicles...");
//         match pay_by_phone.get_vehicles().await {
//             Ok(vehicles) => {
//                 println!("{:?}", vehicles);
//             }
//             Err(e) => {
//                 log::error!("{:?}", e);
//             }
//         }
//     }
// }