use anyhow::{anyhow, Error};
use std::{fmt, str::FromStr};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{de, Deserialize, Deserializer};

use crate::{
    database::MockDB,
    error_handling::{empty_string_as_none, AppError},
    responses::{DepositResponse, Origin, ResponseBody, TransferResponse, WithdrawResponse},
};

#[derive(Debug, Deserialize)]
struct EventParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    r#type: Option<String>,
    origin: Option<String>,
    destination: Option<String>,
    amount: Option<f32>,
}

pub async fn event(
    State(mut state): State<MockDB>,
    Json(payload): Json<EventParams>,
) -> impl IntoResponse {
    let _type = payload.r#type;
    let origin = payload.origin.unwrap_or_default();
    let destination = payload.destination.unwrap_or_default();
    let amount = payload.amount.unwrap_or_default();
    // format!("{payload:?}")
    if let Some(t) = _type {
        match t.as_str() {
            "deposit" => {
                let balance = state.deposit(destination.as_str(), amount).await;
                let deposit = Origin {
                    id: "100".to_string(),
                    balance,
                };

                let response = DepositResponse {
                    destination: deposit,
                };

                let data = serde_json::to_string(&response)?;
                return Ok(Json(data));
            }
            "withdraw" => {
                let result = state.withdraw(origin.as_str(), amount).await;
                if let Ok(balance) = result {
                    let origin = Origin {
                        id: "100".to_string(),
                        balance,
                    };

                    let response = WithdrawResponse { origin };

                    let data = serde_json::to_string(&response)?;

                    return Ok(Json(data));
                }
            }
            "transfer" => {
                let result = state
                    .transfer(origin.as_str(), destination.as_str(), amount)
                    .await;

                if let Ok(r) = result {
                    let origin_balance = state.balance(origin.as_str()).await.unwrap();
                    let origin = Origin {
                        id: origin.to_string(),
                        balance: origin_balance,
                    };
                    let destination_balance = state.balance(destination.as_str()).await.unwrap();
                    let destination = Origin {
                        id: destination.to_string(),
                        balance: destination_balance,
                    };

                    let response = TransferResponse {
                        origin,
                        destination,
                    };

                    let data = serde_json::to_string(&response)?;
                    return Ok(Json(data));
                }
            }
            _ => return Err(AppError::from(anyhow!(""))),
        }
    }

    Ok(Json("0".to_string()))
}
