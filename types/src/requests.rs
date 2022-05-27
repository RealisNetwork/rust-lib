use deadpool::managed::PoolError;
use error_registry::{BaseError, ErrorType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use substrate_api_client::extrinsic::log::trace;
use tokio_postgres::Error;
use error_registry::generated_errors::{Db, GeneratedError};

/// M
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T, Y> {
    pub result: ResponseResult<T, Y>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult<T, Y> {
    pub request: T,
    pub response: ResponseMessage<Y>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseMessage<Y> {
    Left { value: ResponseError },
    Right { value: Y },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseError {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: BaseError<()>,
    pub trace: Option<String>,
    pub data: Option<Value>,
    pub status: Option<i32>,
}

impl From<PoolError<Error>> for ResponseError {
    fn from(error: PoolError<Error>) -> Self {
        Self {
            msg: format!("Fail to get db connection from pool `{:?}`", error),
            error_type: BaseError::new("Connection Error".to_string(),None,None, ErrorType::Generated(GeneratedError::Db(Db::NotFound))),//RealisErrors::Db(Db::ConnectionError),
            status: None,
            trace: None,
            data: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}
