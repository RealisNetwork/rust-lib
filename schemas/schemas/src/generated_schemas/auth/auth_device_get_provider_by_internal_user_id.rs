// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeviceGetProviderByInternalUserIdParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for AuthAuthDeviceGetProviderByInternalUserIdParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") } } impl Agent for AuthAuthDeviceGetProviderByInternalUserIdParams { fn topic () -> & 'static str { "auth_authDevice_getProviderByInternalUserId" } fn method () -> & 'static str { "authDevice_getProviderByInternalUserId" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeviceGetProviderByInternalUserIdReturns (bool) ; impl Schema for AuthAuthDeviceGetProviderByInternalUserIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthDeviceGetProviderByInternalUserIdReturns { fn topic () -> & 'static str { "auth_authDevice_getProviderByInternalUserId" } fn method () -> & 'static str { "authDevice_getProviderByInternalUserId" } fn agent () -> & 'static str { "auth" } }