// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyRefuseOfferParams {
    #[serde(rename = "offerKey")]
    pub offer_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyRefuseOfferParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"offerKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"offerKey\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyRefuseOfferParams {
    fn topic() -> &'static str {
        "cats_lobby_refuseOffer"
    }
    fn method() -> &'static str {
        "lobby_refuseOffer"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyRefuseOfferReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyRefuseOfferReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyRefuseOfferReturns;
impl Schema for CatsLobbyRefuseOfferReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyRefuseOfferReturns {
    fn topic() -> &'static str {
        "cats_lobby_refuseOffer"
    }
    fn method() -> &'static str {
        "lobby_refuseOffer"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
