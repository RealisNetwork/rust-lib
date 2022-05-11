use crate::{
    requests::AuthInfo, schemas::realis_marketplace::change_price_nft::ChangePriceSchema as MarketplaceChangePriceSchema, Amount,
};
use realis_primitives::TokenId;
use rust_lib::json::{
    token_id::{token_id_from_string, token_id_to_string},
    u128::{u128_from_string, u128_to_string},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePriceSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: ChangePriceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePriceSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}

impl From<MarketplaceChangePriceSchema> for ChangePriceSchema {
    fn from(other: MarketplaceChangePriceSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            method: other.method,
            params: ChangePriceSchemaParams {
                token_id: other.params.token_id,
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
