use crate::{
    requests::AuthInfo,
    schemas::realis_adapter::transfer_tokens_to_player::{
        Amount, TransferTokensToPlayerSchema as AdapterTransferTokensToPlayerSchema,
    },
};
use runtime::AccountId;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokensToPlayerSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: TransferTokensToPlayerParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokensToPlayerSchemaParams {
    pub dest: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}

impl From<AdapterTransferTokensToPlayerSchema> for TransferTokensToPlayerSchema {
    fn from(other: AdapterTransferTokensToPlayerSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: TransferTokensToPlayerSchemaParams {
                dest: other.params.dest,
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
