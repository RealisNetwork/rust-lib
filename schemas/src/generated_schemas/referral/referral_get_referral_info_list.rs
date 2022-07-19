// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralInfoListParams { # [serde (rename = "page")] pub page : i64 , # [serde (rename = "perPage")] pub per_page : i64 , # [serde (rename = "userId")] pub user_id : String } impl Schema for ReferralReferralGetReferralInfoListParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"page\",\"perPage\"]}") } } impl Agent for ReferralReferralGetReferralInfoListParams { fn topic () -> & 'static str { "referral_referral_getReferralInfoList" } fn method () -> & 'static str { "referral_getReferralInfoList" } fn agent () -> & 'static str { "referral" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralInfoListReturnsDataParamsParams { # [serde (rename = "registryDate")] pub registry_date : String , # [serde (rename = "nickname")] pub nickname : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "isBanned")] pub is_banned : bool } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralGetReferralInfoListReturns { # [serde (rename = "totalCount")] pub total_count : i64 , # [serde (rename = "data")] pub data : Vec < ReferralReferralGetReferralInfoListReturnsDataParamsParams > } impl Schema for ReferralReferralGetReferralInfoListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"registryDate\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"}},\"required\":[\"userId\",\"nickname\",\"isBanned\",\"registryDate\"]}}},\"required\":[\"totalCount\",\"data\"]}") } } impl Agent for ReferralReferralGetReferralInfoListReturns { fn topic () -> & 'static str { "referral_referral_getReferralInfoList" } fn method () -> & 'static str { "referral_getReferralInfoList" } fn agent () -> & 'static str { "referral" } }