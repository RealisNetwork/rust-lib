// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for UserProfileGetMyBanInfoParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (UserProfileGetMyBanInfoParams) } } # [derive (Debug , Clone , Serialize)] pub struct UserProfileGetMyBanInfoParams ; impl Schema for UserProfileGetMyBanInfoParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for UserProfileGetMyBanInfoParams { fn topic () -> & 'static str { "user_profile_getMyBanInfo" } fn method () -> & 'static str { "profile_getMyBanInfo" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for UserProfileGetMyBanInfoReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (UserProfileGetMyBanInfoReturns) } } # [derive (Debug , Clone , Serialize)] pub struct UserProfileGetMyBanInfoReturns ; impl Schema for UserProfileGetMyBanInfoReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for UserProfileGetMyBanInfoReturns { fn topic () -> & 'static str { "user_profile_getMyBanInfo" } fn method () -> & 'static str { "profile_getMyBanInfo" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } }