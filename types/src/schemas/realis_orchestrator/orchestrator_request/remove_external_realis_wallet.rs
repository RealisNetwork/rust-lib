use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveExternalRalisWalletSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: RemoveExternalRealisWalletSchemaParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveExternalRealisWalletSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
}
