// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthRoleGetAllRolesParams { } impl Schema for AuthRoleGetAllRolesParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{},\"required\":null}") } } impl Agent for AuthRoleGetAllRolesParams { fn topic () -> & 'static str { "auth_role_getAllRoles" } fn method () -> & 'static str { "role_getAllRoles" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthRoleGetAllRolesReturns (pub Vec < String >) ; impl Schema for AuthRoleGetAllRolesReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}") } } impl Agent for AuthRoleGetAllRolesReturns { fn topic () -> & 'static str { "auth_role_getAllRoles" } fn method () -> & 'static str { "role_getAllRoles" } fn agent () -> & 'static str { "auth" } }