use deadpool::managed::PoolError;
use error_registry::{Db, RealisErrors};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Error;

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
    pub error_type: RealisErrors,
    pub trace: Option<String>,
    pub data: Option<Value>,
    pub status: Option<i32>,
}

impl From<PoolError<Error>> for ResponseError {
    fn from(error: PoolError<Error>) -> Self {
        Self {
            msg: format!("Fail to get db connection from pool `{:?}`", error),
            error_type: RealisErrors::Db(Db::ConnectionError),
            trace: None,
            data: None,
            status: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}
