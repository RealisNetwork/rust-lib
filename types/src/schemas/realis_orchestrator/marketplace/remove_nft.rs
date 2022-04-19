use primitives::RequestId;
use crate::requests::AuthInfo;
use realis_primitives::TokenId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftSchema {
    pub id: RequestId,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: RemoveNftParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveNftParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    token_id: TokenId,
}