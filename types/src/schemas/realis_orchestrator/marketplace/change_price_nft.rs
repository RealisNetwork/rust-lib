use realis_primitives::TokenId;
use primitives::RequestId;
use crate::Amount;
use crate::requests::AuthInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangePriceSchema {
    pub id:RequestId,
    #[serde(rename="topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: ChangePriceNftParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangePriceNftParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    token_id: TokenId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    amount: Amount,
}