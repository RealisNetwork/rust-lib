use error_registry::BaseError;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<P> {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    pub params: P,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(rename = "authInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<AuthInfo>,
}

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

impl<Y, D: Debug> From<ResponseMessage<Y, D>> for Result<Y, BaseError<D>> {
    fn from(response_message: ResponseMessage<Y, D>) -> Self {
        match response_message {
            ResponseMessage::Left { value } => Err(value),
            ResponseMessage::Right { value } => Ok(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    #[serde(rename = "mobileApp")]
    MobileApp,
    #[serde(rename = "webSite")]
    WebSite,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "type")]
    pub r#type: StrategyType,
    pub token: Option<String>,
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<u64>,
}

impl Debug for Auth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self
            .token
            .as_ref()
            .map(|token| "*".to_owned().repeat(token.len()));
        f.debug_struct("Auth")
            .field("type", &self.r#type as &dyn Debug)
            .field("token", &a as &dyn Debug)
            .field("deviceId", &self.device_id as &dyn Debug)
            .field("appId", &self.app_id as &dyn Debug)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketRequest<T> {
    pub id: String,
    pub method: String,
    pub agent: String,
    pub params: T,
    pub auth: Auth,
    pub lang: String,
}
