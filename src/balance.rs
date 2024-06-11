use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::database::MockDB;
use crate::error_handling::{empty_string_as_none, AppError, CustomResponse};
use crate::responses::Response;

#[derive(Debug, Deserialize)]
pub struct BalanceQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    account_id: Option<String>,
}

pub async fn balance(
    State(state): State<MockDB>,
    Query(params): Query<BalanceQuery>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.account_id;
    if let Some(_id) = id {
        let res = state.balance(_id.as_str()).await;
        match res {
            Ok(balance) => Ok(CustomResponse {
                status: StatusCode::OK,
                body: Response::Default(balance),
            }),
            Err(_) => Ok(CustomResponse {
                status: StatusCode::NOT_FOUND,
                body: Response::Default(0f32),
            }),
        }
    } else {
        Ok(CustomResponse {
            status: StatusCode::BAD_REQUEST,
            body: Response::Default(0f32),
        })
    }
}
