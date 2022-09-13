// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PromoCodesDeleteCodeParams { # [serde (rename = "code")] pub code : String } impl Schema for PromoCodesDeleteCodeParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}") } } impl Agent for PromoCodesDeleteCodeParams { fn topic () -> & 'static str { "promo_codes_deleteCode" } fn method () -> & 'static str { "codes_deleteCode" } fn agent () -> & 'static str { "promo" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PromoCodesDeleteCodeReturns (pub bool) ; impl Schema for PromoCodesDeleteCodeReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for PromoCodesDeleteCodeReturns { fn topic () -> & 'static str { "promo_codes_deleteCode" } fn method () -> & 'static str { "codes_deleteCode" } fn agent () -> & 'static str { "promo" } }