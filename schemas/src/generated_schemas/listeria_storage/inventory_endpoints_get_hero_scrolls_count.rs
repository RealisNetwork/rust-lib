// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageInventoryEndpointsGetHeroScrollsCountParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ListeriaStorageInventoryEndpointsGetHeroScrollsCountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageInventoryEndpointsGetHeroScrollsCountParams;
impl Schema for ListeriaStorageInventoryEndpointsGetHeroScrollsCountParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetHeroScrollsCountReturns(i32);
impl Schema for ListeriaStorageInventoryEndpointsGetHeroScrollsCountReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}")
    }
}
