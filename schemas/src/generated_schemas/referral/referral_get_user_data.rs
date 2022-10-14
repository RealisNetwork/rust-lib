// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetUserDataParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "appId")] pub app_id : f64 } impl Schema for ReferralReferralGetUserDataParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"appId\",\"userId\"]}") . unwrap () } } impl Agent for ReferralReferralGetUserDataParams { fn topic () -> & 'static str { "referral_referral_getUserData" } fn method () -> & 'static str { "referral_getUserData" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Internal } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetUserDataReturns { # [serde (rename = "refLink" , deserialize_with = "deserialize_to_string")] pub ref_link : String , # [serde (rename = "hasReferrer")] pub has_referrer : bool , # [serde (rename = "refCode" , deserialize_with = "deserialize_to_string")] pub ref_code : String } impl Schema for ReferralReferralGetUserDataReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refLink\":{\"type\":\"string\"},\"hasReferrer\":{\"type\":\"boolean\"},\"refCode\":{\"type\":\"string\"}},\"required\":[\"refLink\",\"refCode\",\"hasReferrer\"]}") } } impl Agent for ReferralReferralGetUserDataReturns { fn topic () -> & 'static str { "referral_referral_getUserData" } fn method () -> & 'static str { "referral_getUserData" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Internal } }