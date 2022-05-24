use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteBalanceByUserIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: DeleteBalanceByUserIdSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteBalanceByUserIdSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
