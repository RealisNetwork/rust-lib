// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsUnitPartsParamsParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "slot")]
    pub slot: String,
    #[serde(rename = "element")]
    pub element: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsAttributesParams {
    #[serde(rename = "defence")]
    pub defence: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "power")]
    pub power: f64,
    #[serde(rename = "speed")]
    pub speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParams {
    #[serde(rename = "unitId")]
    pub unit_id: f64,
    #[serde(rename = "unitParts")]
    pub unit_parts:
        Vec<DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsUnitPartsParamsParams>,
    #[serde(rename = "level")]
    pub level: f64,
    #[serde(rename = "attributes")]
    pub attributes: DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsAttributesParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParams {
    #[serde(rename = "units")]
    pub units: Vec<DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParams>,
}
impl Schema for DragocatsBalancerBalancerEnterQueueParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"units\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"unitParts\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"slot\":{\"type\":\"string\",\"pattern\":\"^(head)|(tail)|(body)$\"},\"element\":{\"type\":\"string\",\"pattern\":\"^(fire)|(water)|(earth)|(air)|(light)|(dark)|(crystal)|(iron)|(energy)$\"}},\"required\":[\"id\",\"element\",\"slot\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"defence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"power\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"speed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"health\",\"power\",\"defence\",\"speed\"]}},\"required\":[\"unitId\",\"attributes\",\"level\",\"unitParts\"]}}},\"required\":[\"units\"]}") . unwrap ()
    }
}
impl Agent for DragocatsBalancerBalancerEnterQueueParams {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_enterQueue"
    }
    fn method() -> &'static str {
        "balancer_enterQueue"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueReturns(pub bool);
impl Schema for DragocatsBalancerBalancerEnterQueueReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for DragocatsBalancerBalancerEnterQueueReturns {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_enterQueue"
    }
    fn method() -> &'static str {
        "balancer_enterQueue"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
