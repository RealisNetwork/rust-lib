use error_registry::BaseError;
use serde::{Deserialize, Deserializer, Serialize};
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

// TODO: Remove this and consider more accurate way to deserialize
fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(d)?;
    let val = opt.unwrap_or_else(T::default);
    Ok(val)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    #[serde(rename = "userId")]
    #[serde(deserialize_with = "null_to_default")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Auth {
    pub token: Option<String>,
}

impl Debug for Auth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self
            .token
            .as_ref()
            .map(|token| "*".to_owned().repeat(token.len()));
        f.debug_struct("Auth")
            .field("token", &a as &dyn Debug)
            .finish()
    }
}
