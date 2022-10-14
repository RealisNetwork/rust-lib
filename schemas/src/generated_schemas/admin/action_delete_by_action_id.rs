// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminActionDeleteByActionIdParams { # [serde (rename = "actionId" , deserialize_with = "deserialize_to_string")] pub action_id : String } impl Schema for AdminActionDeleteByActionIdParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"actionId\":{\"type\":\"string\"}},\"required\":[\"actionId\"]}") . unwrap () } } impl Agent for AdminActionDeleteByActionIdParams { fn topic () -> & 'static str { "admin_action_deleteByActionId" } fn method () -> & 'static str { "action_deleteByActionId" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { AccessLevel :: Private } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminActionDeleteByActionIdReturns (pub bool) ; impl Schema for AdminActionDeleteByActionIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AdminActionDeleteByActionIdReturns { fn topic () -> & 'static str { "admin_action_deleteByActionId" } fn method () -> & 'static str { "action_deleteByActionId" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { AccessLevel :: Private } }