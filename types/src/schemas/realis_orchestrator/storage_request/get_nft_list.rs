use crate::{
    requests::AuthInfo, 
    schemas::storage_service::user_nft_list::GetUserNftListSchema as StorageGetUserNftListSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserNftListSchema {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

impl From<StorageGetUserNftListSchema> for GetUserNftListSchema {
    fn from(other: StorageGetUserNftListSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            auth_info: other.auth_info,
        }
    }
}
