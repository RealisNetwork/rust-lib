// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthAssignProviderAccountToDeviceIdParams { # [serde (rename = "token" , deserialize_with = "deserialize_to_string")] pub token : String } impl Schema for AuthAuthAssignProviderAccountToDeviceIdParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}") . unwrap () } } impl Agent for AuthAuthAssignProviderAccountToDeviceIdParams { fn topic () -> & 'static str { "auth_auth_assignProviderAccountToDeviceId" } fn method () -> & 'static str { "auth_assignProviderAccountToDeviceId" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthAssignProviderAccountToDeviceIdReturns (pub bool) ; impl Schema for AuthAuthAssignProviderAccountToDeviceIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthAssignProviderAccountToDeviceIdReturns { fn topic () -> & 'static str { "auth_auth_assignProviderAccountToDeviceId" } fn method () -> & 'static str { "auth_assignProviderAccountToDeviceId" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }