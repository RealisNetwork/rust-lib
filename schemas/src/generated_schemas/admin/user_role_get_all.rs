// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AdminUserRoleGetAllParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (AdminUserRoleGetAllParams) } } # [derive (Debug , Clone , Serialize)] pub struct AdminUserRoleGetAllParams ; impl Schema for AdminUserRoleGetAllParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for AdminUserRoleGetAllParams { fn topic () -> & 'static str { "admin_userRole_getAll" } fn method () -> & 'static str { "userRole_getAll" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { AccessLevel :: Private } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleGetAllReturnsParamsRoleParams { # [serde (rename = "name" , deserialize_with = "deserialize_to_string")] pub name : String , # [serde (rename = "id")] pub id : f64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleGetAllReturnsParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "id")] pub id : f64 , # [serde (rename = "isActive")] pub is_active : bool , # [serde (rename = "role")] pub role : AdminUserRoleGetAllReturnsParamsRoleParams } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleGetAllReturns (pub Vec < AdminUserRoleGetAllReturnsParams >) ; impl Schema for AdminUserRoleGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isActive\":{\"type\":\"boolean\"},\"role\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\"]}},\"required\":[\"id\",\"userId\",\"isActive\",\"role\"]}}") } } impl Agent for AdminUserRoleGetAllReturns { fn topic () -> & 'static str { "admin_userRole_getAll" } fn method () -> & 'static str { "userRole_getAll" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { AccessLevel :: Private } }