use std::{fmt, str::FromStr};

use axum::extract::{Query, State};
use serde::{de, Deserialize, Deserializer};

use crate::database::MockDB;
use crate::error_handling::empty_string_as_none;

#[derive(Debug, Deserialize)]
struct BalanceQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    account_id: Option<String>,
}

pub async fn balance(State(state): State<MockDB>, Query(params): Query<BalanceQuery>) -> String {
    let id = params.account_id;
    if let Some(_id) = id {
        let res = state.balance(_id.as_str()).await;
        if let Ok(x) = res {
            x.to_string()
        } else {
            0.to_string()
        }
    } else {
        0.to_string()
    }
}
