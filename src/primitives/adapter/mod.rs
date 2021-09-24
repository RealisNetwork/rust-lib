pub mod db;
pub mod request;
pub mod response;
pub mod types;

pub use crate::primitives::adapter::request::{Id, Request};
use realis_primitives::TokenId;
use runtime::AccountId;
use sp_core::H256;
use thiserror::Error;

pub type UserId = String;
pub type TxHash = H256;
pub type Balance = u128;

#[derive(Debug, Clone)]
pub enum Message {
    Realis(RealisRequest),
    Responder(ResponderRequest),
    Terminate(String),
}

#[derive(Debug, Clone)]
pub enum RealisRequest {
    Request(Request),
    Terminate,
}

#[derive(Debug, Clone)]
pub enum DBRequest {
    Request(Request),
    RequestProcessed(Id, bool),
    UserCreateFailed(UserId),
    Terminate,
}

#[derive(Debug, Clone)]
pub enum ResponderRequest {
    Ok(Request, ResponderParams),
    Err(Request, String),
    Terminate,
}

#[derive(Debug, Clone)]
pub enum ResponderParams {
    CreateWallet,
    Balance(Balance),
    Tokens(Vec<TokenId>),
    Hash(TxHash),
    AccountId(AccountId),
}

// TODO thiserror
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while trying use realis connection: {0}")]
    Api(substrate_api_client::ApiClientError),
    #[error("Error while send throw channel!")]
    SendError,
    #[error("Cannot send Message: {0}")]
    SendMessage(tokio::sync::mpsc::error::SendError<Message>),
    #[error("Cannot send Request: {0}")]
    Send(tokio::sync::mpsc::error::SendError<Request>),
    #[error("Error while trying use tokio_postgres: {0}")]
    Postgres(tokio_postgres::Error),
    #[error("Error while trying parse: {0}")]
    SerdeJSON(serde_json::error::Error),
    #[error("Disconnected from Database!")]
    Disconnected,
    #[error("Cannot found this file in this path {0}!")]
    FileNotFound(String),
    #[error("Cannot decode this value!")]
    CannotDecode,
    #[error("User not found!")]
    NotInteresting,
    #[error("Error in nats: {0}")]
    NatsError(ratsio::error::RatsioError),
}
