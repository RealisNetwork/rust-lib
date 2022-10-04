// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminPermissionAddParams { # [serde (rename = "roleName")] pub role_name : String , # [serde (rename = "permissionName")] pub permission_name : Vec < String > } impl Schema for AdminPermissionAddParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"},\"permissionName\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"permissionName\",\"roleName\"]}") . unwrap () } } impl Agent for AdminPermissionAddParams { fn topic () -> & 'static str { "admin_permission_add" } fn method () -> & 'static str { "permission_add" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminPermissionAddReturns (pub bool) ; impl Schema for AdminPermissionAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AdminPermissionAddReturns { fn topic () -> & 'static str { "admin_permission_add" } fn method () -> & 'static str { "permission_add" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } }