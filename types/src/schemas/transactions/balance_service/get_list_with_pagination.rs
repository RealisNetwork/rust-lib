use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetListWithPaginationSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: GetListWithPaginationSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetListWithPaginationSchemaParams {
    pub page: u32,
}
