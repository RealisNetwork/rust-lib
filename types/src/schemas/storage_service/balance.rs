use crate::{
    requests::AuthInfo, schemas::realis_orchestrator::storage_request::get_balance::GetBalanceSchema as OrchestratorGetBalanceSchema,
};
use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[allow(clippy::pedantic)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: Option<GetBalanceSchemaParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl GetBalanceSchema {
    pub fn new(other: OrchestratorGetBalanceSchema, account_id: AccountId) -> Self {
        let params: Option<GetBalanceSchemaParams> = Option::from(GetBalanceSchemaParams { account_id: account_id });
        Self {
            id: other.id,
            topic_res: other.topic_res,
            auth_info: other.auth_info,
            params: params,
        }
    }
}
