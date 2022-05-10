use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorGetNftListSchema {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
