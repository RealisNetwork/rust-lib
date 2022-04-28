use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;
use rust_lib::json::uuid::{uuid_from_string, uuid_to_string};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: NftMintedSuccessParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessParams {
    #[serde(rename = "txId")]
    pub hash: Hash,
    #[serde(serialize_with = "uuid_to_string")]
    #[serde(deserialize_with = "uuid_from_string")]
    pub uuid: uuid::Uuid,
}