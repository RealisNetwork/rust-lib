// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigDeleteParams { # [serde (rename = "id")] pub id : f64 } impl Schema for StatusConfigDeleteParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") } } impl Agent for StatusConfigDeleteParams { fn topic () -> & 'static str { "status_config_delete" } fn method () -> & 'static str { "config_delete" } fn agent () -> & 'static str { "status" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigDeleteReturns (pub bool) ; impl Schema for StatusConfigDeleteReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for StatusConfigDeleteReturns { fn topic () -> & 'static str { "status_config_delete" } fn method () -> & 'static str { "config_delete" } fn agent () -> & 'static str { "status" } }