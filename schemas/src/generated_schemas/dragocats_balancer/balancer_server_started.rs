// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerServerStartedParams {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "port")]
    pub port: f64,
}
impl Schema for DragocatsBalancerBalancerServerStartedParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"clientId\":{\"type\":\"string\"},\"address\":{\"type\":\"string\"},\"port\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"clientId\",\"address\",\"port\"]}")
    }
}
impl Agent for DragocatsBalancerBalancerServerStartedParams {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_serverStarted"
    }
    fn method() -> &'static str {
        "balancer_serverStarted"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
impl<'de> Deserialize<'de> for DragocatsBalancerBalancerServerStartedReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragocatsBalancerBalancerServerStartedReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsBalancerBalancerServerStartedReturns;
impl Schema for DragocatsBalancerBalancerServerStartedReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragocatsBalancerBalancerServerStartedReturns {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_serverStarted"
    }
    fn method() -> &'static str {
        "balancer_serverStarted"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
