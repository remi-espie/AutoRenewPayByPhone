mod config;
mod middleware;
mod paybyphone;
mod types;

use crate::config::Accounts;
use crate::middleware::auth_middleware;
use axum::extract::{Query, State};
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use dotenvy::dotenv;
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Rémi Espié", about, long_about = None)]
struct Args {
    /// The port the application will listen on. Default is 3000.
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Bearer token for authentication. Can be set through the BEARER environment variable.
    #[arg(short, long, env)]
    bearer: String,
}

#[derive(Deserialize)]
struct AccountName {
    name: String,
}

#[derive(Deserialize)]
struct Parking {
    name: String,
    duration: i16,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let args = Args::parse();
    let bearer_token = Arc::new(args.bearer.clone());

    log::info!("Reading user config...");
    let config = Arc::new(config::read("config.yaml").unwrap_or_else(|e| panic!("{:?}", e)));

    let app = Router::new()
        .route("/healthz", get(()))
        .route("/accounts", get(get_accounts))
        .route("/quote", get(get_quote))
        .route("/park", post(park))
        .route("/check", get(get_sessions))
        .route("/vehicles", get(get_vehicles))
        .with_state(config)
        .layer(from_fn(move |req, next| {
            auth_middleware(req, next, bearer_token.clone())
        }));
    // .route("/cancel", post(|| async { Json(StatusCode::OK) }))

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    log::info!("Listening on 0.0.0.0:{}", args.port);
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    })
    .await
    .unwrap();
}

async fn initalize_pay_by_phone(
    config: Arc<Accounts>,
    account_name: String,
) -> Result<paybyphone::PayByPhone, Box<dyn Error + Send + Sync>> {
    match config.accounts.iter().find(|a| a.name == account_name) {
        Some(account) => {
            let mut pay_by_phone = paybyphone::PayByPhone::new(
                account.plate.clone(),
                account.lot,
                account.pay_by_phone.login.clone(),
                account.pay_by_phone.password.clone(),
                account.pay_by_phone.payment_account_id.clone(),
            );
            match pay_by_phone.init().await {
                Ok(_) => {
                    log::info!("PayByPhone initialized");
                    Ok(pay_by_phone)
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    Err(Box::from(format!("Failed to initialize PayByPhone: {}", e)))
                }
            }
        }
        None => Err(Box::from("Account not found")),
    }
}

async fn get_sessions(
    State(config): State<Arc<Accounts>>,
    Query(account_name): Query<AccountName>,
) -> impl IntoResponse {
    match initalize_pay_by_phone(config, account_name.name).await {
        Ok(pay_by_phone) => {
            log::info!("Checking...");
            match pay_by_phone.check().await {
                Ok(sessions) => (StatusCode::OK, Json(sessions)).into_response(),
                Err(e) => {
                    log::error!("{:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to initialize PayByPhone: {}", e)),
        )
            .into_response(),
    }
}

async fn get_vehicles(
    State(config): State<Arc<Accounts>>,
    Query(account_name): Query<AccountName>,
) -> impl IntoResponse {
    match initalize_pay_by_phone(config, account_name.name).await {
        Ok(pay_by_phone) => {
            log::info!("Getting vehicles...");
            match pay_by_phone.get_vehicles().await {
                Ok(vehicles) => (StatusCode::OK, Json(vehicles)).into_response(),
                Err(e) => {
                    log::error!("{:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to initialize PayByPhone: {}", e)),
        )
            .into_response(),
    }
}

async fn get_quote(
    State(config): State<Arc<Accounts>>,
    Query(parking): Query<Parking>,
) -> impl IntoResponse {
    match initalize_pay_by_phone(config, parking.name).await {
        Ok(pay_by_phone) => {
            log::info!("Getting quote...");
            match pay_by_phone.quote(parking.duration).await {
                Ok(quote) => (StatusCode::OK, Json(quote)).into_response(),
                Err(e) => {
                    log::error!("{:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to initialize PayByPhone: {}", e)),
        )
            .into_response(),
    }
}

async fn park(
    State(config): State<Arc<Accounts>>,
    Json(parking): Json<Parking>,
) -> impl IntoResponse {
    match initalize_pay_by_phone(config, parking.name).await {
        Ok(pay_by_phone) => {
            log::info!("Getting vehicles...");
            match pay_by_phone.park(parking.duration).await {
                Ok(session) => (StatusCode::ACCEPTED, Json(session)).into_response(),
                Err(e) => {
                    log::error!("{:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to initialize PayByPhone: {}", e)),
        )
            .into_response(),
    }
}

async fn get_accounts(State(config): State<Arc<Accounts>>) -> impl IntoResponse {
    (StatusCode::OK, Json(config)).into_response()
}
