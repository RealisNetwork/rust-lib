// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RefundBalancesGetAllUnavailableParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for RefundBalancesGetAllUnavailableParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for RefundBalancesGetAllUnavailableParams { fn topic () -> & 'static str { "refund_balances_getAllUnavailable" } fn method () -> & 'static str { "balances_getAllUnavailable" } fn agent () -> & 'static str { "refund" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RefundBalancesGetAllUnavailableReturns { # [serde (rename = "ETH")] pub eth : Option < String > , # [serde (rename = "LIS")] pub lis : Option < String > } impl Schema for RefundBalancesGetAllUnavailableReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"ETH\":{\"type\":\"string\"},\"LIS\":{\"type\":\"string\"}}}") } } impl Agent for RefundBalancesGetAllUnavailableReturns { fn topic () -> & 'static str { "refund_balances_getAllUnavailable" } fn method () -> & 'static str { "balances_getAllUnavailable" } fn agent () -> & 'static str { "refund" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }