use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceByUserIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: GetBalanceByUserIdSchemaParams,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceByUserIdSchemaParams {
    pub currency: String,
}
