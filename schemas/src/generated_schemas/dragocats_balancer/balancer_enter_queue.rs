// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsAttributesParams {
    #[serde(rename = "power")]
    pub power: f64,
    #[serde(rename = "speed")]
    pub speed: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "defence")]
    pub defence: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsUnitPartsParamsParams {
    #[serde(rename = "element")]
    pub element: String,
    #[serde(rename = "slot")]
    pub slot: String,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParams {
    #[serde(rename = "level")]
    pub level: f64,
    #[serde(rename = "attributes")]
    pub attributes: DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsAttributesParams,
    #[serde(rename = "unitParts")]
    pub unit_parts:
        Vec<DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParamsUnitPartsParamsParams>,
    #[serde(rename = "unitId")]
    pub unit_id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerEnterQueueParams {
    #[serde(rename = "units")]
    pub units: Vec<DragocatsBalancerBalancerEnterQueueParamsUnitsParamsParams>,
}
impl Schema for DragocatsBalancerBalancerEnterQueueParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"units\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"power\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"speed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"defence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"health\",\"power\",\"defence\",\"speed\"]},\"unitParts\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"element\":{\"type\":\"string\",\"pattern\":\"^(fire)|(water)|(earth)|(air)|(light)|(dark)|(crystal)|(iron)|(energy)$\"},\"slot\":{\"type\":\"string\",\"pattern\":\"^(head)|(tail)|(body)$\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"element\",\"slot\"]}},\"unitId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"unitId\",\"attributes\",\"level\",\"unitParts\"]}}},\"required\":[\"units\"]}") . unwrap ()
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
