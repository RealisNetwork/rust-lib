// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthRoleDeleteParams { # [serde (rename = "roleName")] pub role_name : String } impl Schema for AuthRoleDeleteParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}") } } impl Agent for AuthRoleDeleteParams { fn topic () -> & 'static str { "auth_role_delete" } fn method () -> & 'static str { "role_delete" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthRoleDeleteReturns (pub bool) ; impl Schema for AuthRoleDeleteReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthRoleDeleteReturns { fn topic () -> & 'static str { "auth_role_delete" } fn method () -> & 'static str { "role_delete" } fn agent () -> & 'static str { "auth" } }