use rust_lib::json::serde_uuid::{uuid_from_string, uuid_to_string};
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: NftMintedSuccessSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessSchemaParams {
    #[serde(rename = "txId")]
    pub hash: Hash,
    #[serde(serialize_with = "uuid_to_string")]
    #[serde(deserialize_with = "uuid_from_string")]
    pub uuid: uuid::Uuid,
}
