use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtrinsicConfirmationSchema {
    pub id: String,
    pub params: Value,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
