use std::fmt::Debug;
use deadpool::managed::PoolError;
use error_registry::{
    custom_errors::{CustomErrorType, CustomErrorType::Db, Db::ConnectionError},
    BaseError, ErrorType,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Error;

/// M
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T, Y, D: Debug> {
    pub result: ResponseResult<T, Y, D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult<T, Y, D: Debug> {
    pub request: T,
    pub response: ResponseMessage<Y, D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseMessage<Y, D: Debug> {
    Left { value: BaseError<D> },
    Right { value: Y },
}

// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct ResponseError {
//     pub msg: String,
//     #[serde(rename = "type")]
//     pub error_type: BaseError<()>,
//     pub trace: Option<String>,
//     pub data: Option<Value>,
//     pub status: Option<i32>,
// }

// impl From<PoolError<Error>> for ResponseError {
//     fn from(error: PoolError<Error>) -> Self {
//         Self {
//             msg: format!("Fail to get db connection from pool `{:?}`", error),
//             error_type: BaseError::from(CustomErrorType::Db(ConnectionError)),
//             status: None,
//             trace: None,
//             data: None,
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}
