// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AuthTwoFactorIsRequiredTwoFaParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AuthTwoFactorIsRequiredTwoFaParams) } } # [derive (Debug , Clone , Serialize)] pub struct AuthTwoFactorIsRequiredTwoFaParams ; impl Schema for AuthTwoFactorIsRequiredTwoFaParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AuthTwoFactorIsRequiredTwoFaParams { fn topic () -> & 'static str { "auth_twoFactor_isRequiredTwoFA" } fn method () -> & 'static str { "twoFactor_isRequiredTwoFA" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthTwoFactorIsRequiredTwoFaReturns (pub bool) ; impl Schema for AuthTwoFactorIsRequiredTwoFaReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthTwoFactorIsRequiredTwoFaReturns { fn topic () -> & 'static str { "auth_twoFactor_isRequiredTwoFA" } fn method () -> & 'static str { "twoFactor_isRequiredTwoFA" } fn agent () -> & 'static str { "auth" } }