use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPersonalTypesSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: GetPersonalTypesSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPersonalTypesSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
