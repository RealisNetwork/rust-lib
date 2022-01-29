use crate::requests::AuthInfo;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletSchema {
    pub id: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}