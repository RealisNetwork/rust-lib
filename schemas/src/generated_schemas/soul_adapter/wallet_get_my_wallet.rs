// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for SoulAdapterWalletGetMyWalletParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(SoulAdapterWalletGetMyWalletParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct SoulAdapterWalletGetMyWalletParams;
impl Schema for SoulAdapterWalletGetMyWalletParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletGetMyWalletReturns(String);
impl Schema for SoulAdapterWalletGetMyWalletReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}