// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminActionUndoParams { # [serde (rename = "actionId")] pub action_id : String } impl Schema for AdminActionUndoParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"actionId\":{\"type\":\"string\"}},\"required\":[\"actionId\"]}") . unwrap () } } impl Agent for AdminActionUndoParams { fn topic () -> & 'static str { "admin_action_undo" } fn method () -> & 'static str { "action_undo" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminActionUndoReturns (pub bool) ; impl Schema for AdminActionUndoReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AdminActionUndoReturns { fn topic () -> & 'static str { "admin_action_undo" } fn method () -> & 'static str { "action_undo" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } }