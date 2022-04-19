use primitives::RequestId;
use crate::requests::AuthInfo;
use realis_primitives::TokenId;

pub type Amount = u128;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellNftSchema {
    pub id: RequestId,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: SellNftParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SellNftParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    token_id: TokenId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    amount: Amount,
}