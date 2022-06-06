use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNumWithFilterSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: GetNumWithFilterSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNumWithFilterSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub credit: Option<String>,
    pub debit: Option<String>,
}
