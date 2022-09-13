// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for UserProfileGetAllUsersSubscribedToMailingParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (UserProfileGetAllUsersSubscribedToMailingParams) } } # [derive (Debug , Clone , Serialize)] pub struct UserProfileGetAllUsersSubscribedToMailingParams ; impl Schema for UserProfileGetAllUsersSubscribedToMailingParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for UserProfileGetAllUsersSubscribedToMailingParams { fn topic () -> & 'static str { "user_profile_getAllUsersSubscribedToMailing" } fn method () -> & 'static str { "profile_getAllUsersSubscribedToMailing" } fn agent () -> & 'static str { "user" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetAllUsersSubscribedToMailingReturns (pub Vec < String >) ; impl Schema for UserProfileGetAllUsersSubscribedToMailingReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}") } } impl Agent for UserProfileGetAllUsersSubscribedToMailingReturns { fn topic () -> & 'static str { "user_profile_getAllUsersSubscribedToMailing" } fn method () -> & 'static str { "profile_getAllUsersSubscribedToMailing" } fn agent () -> & 'static str { "user" } }