use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetUserBalancesSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: GetUserBalancesSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetUserBalancesSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String, // TODO maybe redundant and should get user_id from auth_info
}
