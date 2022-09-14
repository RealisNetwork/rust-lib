use error_registry::BaseError;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

impl<Y, D: Debug> From<BaseError<D>> for ResponseMessage<Y, D> {
    fn from(base_error: BaseError<D>) -> Self {
        ResponseMessage::Left { value: base_error }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
#[serde(tag = "type")]
pub enum Auth {
    #[serde(rename = "mobileApp")]
    MobileApp(MobileAuth),
    #[serde(rename = "webSite")]
    WebSite {
        #[serde(skip_serializing_if = "Option::is_none")]
        token: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "appId")]
    pub app_id: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketProcessedRequest<T> {
    pub id: String,
    pub method: String,
    pub agent: String,
    pub params: T,
    pub lang: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    #[serde(rename = "topicResponse")]
    pub topic_response: String,
    pub auth: Auth,
}

impl<T> From<SocketRequest<T>> for SocketProcessedRequest<T> {
    fn from(socket_request: SocketRequest<T>) -> Self {
        Self {
            id: socket_request.id,
            method: socket_request.method,
            agent: socket_request.agent,
            params: socket_request.params,
            auth: socket_request.auth,
            lang: socket_request.lang,
            auth_info: AuthInfo {
                user_id: None,
                address: None,
                continent: None,
            },
            client_id: "".to_string(),
            topic_response: "".to_string(),
        }
    }
}
