use realis_primitives::TokenId;
use runtime::AccountId;
use crate::{
    requests::AuthInfo,
    schemas::realis_adapter::transfer_nft_to_player::TransferNftToPlayerSchema as AdapterTransferNftToPlayerSchema
};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};
use rust_lib::blockchain::cold_wallets::RealisGameApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftToPlayerSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: TransferNftToPlayerParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftToPlayerParams {
    pub dest: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
}

impl From<AdapterTransferNftToPlayerSchema> for TransferNftToPlayerSchema{
    fn from(other: AdapterTransferNftToPlayerSchema) -> Self {
        TransferNftToPlayerSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: TransferNftToPlayerParams {
                dest: other.params.dest,
                token_id: other.params.token_id,
            },
            auth_info: other.auth_info,
        }
    }
}
