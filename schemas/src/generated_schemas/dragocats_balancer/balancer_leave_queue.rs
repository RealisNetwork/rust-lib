// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerLeaveQueueParams {}
impl Schema for DragocatsBalancerBalancerLeaveQueueParams {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
    }
}
impl Agent for DragocatsBalancerBalancerLeaveQueueParams {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_leaveQueue"
    }
    fn method() -> &'static str {
        "balancer_leaveQueue"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsBalancerBalancerLeaveQueueReturns(bool);
impl Schema for DragocatsBalancerBalancerLeaveQueueReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for DragocatsBalancerBalancerLeaveQueueReturns {
    fn topic() -> &'static str {
        "dragocats-balancer_balancer_leaveQueue"
    }
    fn method() -> &'static str {
        "balancer_leaveQueue"
    }
    fn agent() -> &'static str {
        "dragocats-balancer"
    }
}
