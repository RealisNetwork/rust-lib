use serde::{Serialize, Deserialize};
use serde_json::Value;

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
pub enum ResponseMessage<T> {
    Left { value: ResponseError },
    Right { value: T },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub trace: Option<String>,
    pub data: Option<Value>,
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub address: Option<String>,
    pub continent: Option<String>,
}
