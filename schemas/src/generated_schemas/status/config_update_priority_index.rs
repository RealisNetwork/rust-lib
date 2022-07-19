// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigUpdatePriorityIndexParamsConfigParamsParamsParams { # [serde (rename = "priorityIndex")] pub priority_index : i64 , # [serde (rename = "id")] pub id : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigUpdatePriorityIndexParams { # [serde (rename = "configParams")] pub config_params : Vec < StatusConfigUpdatePriorityIndexParamsConfigParamsParamsParams > } impl Schema for StatusConfigUpdatePriorityIndexParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"configParams\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"priorityIndex\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"priorityIndex\"]}}},\"required\":[\"configParams\"]}") } } impl Agent for StatusConfigUpdatePriorityIndexParams { fn topic () -> & 'static str { "status_config_updatePriorityIndex" } fn method () -> & 'static str { "config_updatePriorityIndex" } fn agent () -> & 'static str { "status" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigUpdatePriorityIndexReturns (bool) ; impl Schema for StatusConfigUpdatePriorityIndexReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for StatusConfigUpdatePriorityIndexReturns { fn topic () -> & 'static str { "status_config_updatePriorityIndex" } fn method () -> & 'static str { "config_updatePriorityIndex" } fn agent () -> & 'static str { "status" } }