use crate::{requests::AuthInfo, schemas::realis_marketplace::buy_nft::BuyNftSchema as MarketplaceBuyNftSchema};
use realis_primitives::TokenId;
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: BuyNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNftSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
}

impl From<MarketplaceBuyNftSchema> for BuyNftSchema {
    fn from(other: MarketplaceBuyNftSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            method: other.method,
            params: BuyNftSchemaParams {
                token_id: other.params.token_id,
            },
            auth_info: other.auth_info,
        }
    }
}
