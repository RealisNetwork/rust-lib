// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralsParams { # [serde (rename = "appId")] pub app_id : f64 , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for ReferralReferralGetReferralsParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appId\"]}") . unwrap () } } impl Agent for ReferralReferralGetReferralsParams { fn topic () -> & 'static str { "referral_referral_getReferrals" } fn method () -> & 'static str { "referral_getReferrals" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Private } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralsReturnsReferralsParamsParams { # [serde (rename = "appId")] pub app_id : f64 , # [serde (rename = "nickname" , deserialize_with = "deserialize_to_string")] pub nickname : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams { # [serde (rename = "date" , deserialize_with = "deserialize_to_string")] pub date : String , # [serde (rename = "nickname" , deserialize_with = "deserialize_to_string")] pub nickname : String , # [serde (rename = "amount" , deserialize_with = "deserialize_to_string")] pub amount : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralsReturns { # [serde (rename = "referrals")] pub referrals : Vec < ReferralReferralGetReferralsReturnsReferralsParamsParams > , # [serde (rename = "referralTransactions")] pub referral_transactions : Vec < ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams > } impl Schema for ReferralReferralGetReferralsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referrals\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"}},\"required\":[\"nickname\",\"appId\"]}},\"referralTransactions\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"date\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"date\",\"nickname\",\"amount\"]}}},\"required\":[\"referrals\",\"referralTransactions\"]}") } } impl Agent for ReferralReferralGetReferralsReturns { fn topic () -> & 'static str { "referral_referral_getReferrals" } fn method () -> & 'static str { "referral_getReferrals" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Private } }