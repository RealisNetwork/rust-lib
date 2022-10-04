// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminRoleGetRoleByNameParams { # [serde (rename = "roleName")] pub role_name : String } impl Schema for AdminRoleGetRoleByNameParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}") . unwrap () } } impl Agent for AdminRoleGetRoleByNameParams { fn topic () -> & 'static str { "admin_role_getRoleByName" } fn method () -> & 'static str { "role_getRoleByName" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminRoleGetRoleByNameReturns { # [serde (rename = "name")] pub name : String , # [serde (rename = "id")] pub id : f64 } impl Schema for AdminRoleGetRoleByNameReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\"]}") } } impl Agent for AdminRoleGetRoleByNameReturns { fn topic () -> & 'static str { "admin_role_getRoleByName" } fn method () -> & 'static str { "role_getRoleByName" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } }