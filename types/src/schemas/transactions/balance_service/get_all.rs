use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAllSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: GetAllSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAllSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
}
