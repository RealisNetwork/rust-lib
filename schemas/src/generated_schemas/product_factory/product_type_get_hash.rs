// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductTypeGetHashParams {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "orchestratorId")]
    pub orchestrator_id: f64,
}
impl Schema for ProductFactoryProductTypeGetHashParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"orchestratorId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"type\",\"orchestratorId\"]}")
    }
}
impl Agent for ProductFactoryProductTypeGetHashParams {
    fn topic() -> &'static str {
        "productFactory_productType_getHash"
    }
    fn method() -> &'static str {
        "productType_getHash"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
impl<'de> Deserialize<'de> for ProductFactoryProductTypeGetHashReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
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
impl Agent for ProductFactoryProductTypeGetHashReturns {
    fn topic() -> &'static str {
        "productFactory_productType_getHash"
    }
    fn method() -> &'static str {
        "productType_getHash"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
