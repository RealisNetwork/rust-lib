use crate::{requests::AuthInfo, schemas::withdraw_bsc::binance_nft_request::BinanceNftRequestSchema};
use json::token_id::{token_id_from_string, token_id_to_string};
use realis_primitives::TokenId;
use runtime::AccountId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWithdrawNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: BinanceWithdrawNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceWithdrawNftSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}

impl From<BinanceNftRequestSchema> for BinanceWithdrawNftSchema {
    fn from(other: BinanceNftRequestSchema) -> Self {
        BinanceWithdrawNftSchema {
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            params: BinanceWithdrawNftSchemaParams {
                account_id: other.params.account_id.clone(),
                token_id: other.params.token_id,
                from_account_id: other.params.from_account_id.clone(),
            },
            auth_info: other.auth_info.clone(),
        }
    }
}
