// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleAddParams { # [serde (rename = "internalUserId")] pub internal_user_id : String , # [serde (rename = "roleName")] pub role_name : String } impl Schema for AdminUserRoleAddParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"},\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\",\"internalUserId\"]}") } } impl Agent for AdminUserRoleAddParams { fn topic () -> & 'static str { "admin_userRole_add" } fn method () -> & 'static str { "userRole_add" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleAddReturns (bool) ; impl Schema for AdminUserRoleAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AdminUserRoleAddReturns { fn topic () -> & 'static str { "admin_userRole_add" } fn method () -> & 'static str { "userRole_add" } fn agent () -> & 'static str { "admin" } }