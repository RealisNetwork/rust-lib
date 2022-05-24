use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AddProductHashSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: AddProductHashSchemaParams,
}

pub struct AddProductHashSchemaParams {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "hashId")]
    pub hash_id: String,
}
