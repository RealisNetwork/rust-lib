use crate::{
    requests::AuthInfo, schemas::transactions::balance_service::delete_balance_by_user_id::DeleteBalanceByUserIdSchemaParams,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteProductByUserIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: DeleteBalanceByUserIdSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteProductByUserIdSchemaParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
