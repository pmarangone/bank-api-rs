use std::collections::HashMap;
use std::sync::Arc;
use std::{convert::Infallible, time::Duration};

use anyhow::{anyhow, Error};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::routing::post;
use axum::Json;
use axum::{
    body::{Body, Bytes},
    extract::{Query, State},
    http::{HeaderName, HeaderValue},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::Client;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod balance;
mod database;
mod error_handling;
mod event;
mod responses;

use balance::balance;
use database::MockDB;
use error_handling::AppError;
use event::{event, reset};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_reqwest_response=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let balances: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mock = MockDB { balances };

    let app = Router::new()
        .route("/balance", get(balance))
        .route("/reset", post(reset))
        .route("/event", post(event))
        .with_state(mock)
        .layer(TraceLayer::new_for_http().on_body_chunk(
            |chunk: &Bytes, _latency: Duration, _span: &Span| {
                tracing::debug!("streaming {} bytes", chunk.len());
            },
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
