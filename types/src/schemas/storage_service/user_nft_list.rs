use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::storage_request::get_nft_list::GetUserNftListSchema as OrchestratorGetNftListSchema,
};
use runtime::AccountId;
use serde::{Deserialize, Serialize};

// Schema for serialize/deserialize json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserNftListSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: GetUserNftListSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetUserNftListSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl GetUserNftListSchema {
    pub fn new(other: OrchestratorGetNftListSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: GetUserNftListSchemaParams { account_id: account_id },
            auth_info: other.auth_info,
        }
    }
}
