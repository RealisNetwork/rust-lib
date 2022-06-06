use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BurnProductSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: BurnProductSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BurnProductSchemaParams {
    #[serde(rename = "productId")]
    pub product_id: String,
}
