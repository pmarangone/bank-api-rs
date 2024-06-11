use serde::Serialize;

#[derive(Serialize)]
pub struct Origin {
    pub id: String,
    pub balance: f32,
}

#[derive(Serialize)]
pub struct DepositResponse {
    pub destination: Origin,
}

#[derive(Serialize)]
pub struct WithdrawResponse {
    pub origin: Origin,
}

#[derive(Serialize)]
pub struct TransferResponse {
    pub origin: Origin,
    pub destination: Origin,
}

#[derive(Serialize)]
pub struct OKResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Deposit(DepositResponse),
    Withdraw(WithdrawResponse),
    Transfer(TransferResponse),
    Default(f32),
}
