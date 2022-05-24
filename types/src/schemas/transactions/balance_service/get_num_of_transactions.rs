use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNumOfTransactionsSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: GetNumOfTransactionsSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNumOfTransactionsSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String, // TODO maybe redundant and should get user_id from auth_info
}
