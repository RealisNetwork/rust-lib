// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductTypeGetHashParams {
    #[serde(rename = "orchestratorId")]
    pub orchestrator_id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
}
impl Schema for ProductFactoryProductTypeGetHashParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"orchestratorId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"type\":{\"type\":\"string\"}},\"required\":[\"type\",\"orchestratorId\"]}")
    }
}
impl<'de> Deserialize<'de> for ProductFactoryProductTypeGetHashReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ProductFactoryProductTypeGetHashReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ProductFactoryProductTypeGetHashReturns;
impl Schema for ProductFactoryProductTypeGetHashReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}