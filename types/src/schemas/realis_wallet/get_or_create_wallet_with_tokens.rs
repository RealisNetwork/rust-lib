use crate::requests::{AuthInfo, Params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletWithTokensSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: Params,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
