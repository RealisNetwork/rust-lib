// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeviceGetKeycloakProviderIdByUserIdParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for AuthAuthDeviceGetKeycloakProviderIdByUserIdParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") } } impl Agent for AuthAuthDeviceGetKeycloakProviderIdByUserIdParams { fn topic () -> & 'static str { "auth_authDevice_getKeycloakProviderIdByUserId" } fn method () -> & 'static str { "authDevice_getKeycloakProviderIdByUserId" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeviceGetKeycloakProviderIdByUserIdReturns (String) ; impl Schema for AuthAuthDeviceGetKeycloakProviderIdByUserIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for AuthAuthDeviceGetKeycloakProviderIdByUserIdReturns { fn topic () -> & 'static str { "auth_authDevice_getKeycloakProviderIdByUserId" } fn method () -> & 'static str { "authDevice_getKeycloakProviderIdByUserId" } fn agent () -> & 'static str { "auth" } }