// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeIsExistParams { # [serde (rename = "personalType")] pub personal_type : String } impl Schema for ProductFactoryProductTypeIsExistParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap () } } impl Agent for ProductFactoryProductTypeIsExistParams { fn topic () -> & 'static str { "productFactory_productType_isExist" } fn method () -> & 'static str { "productType_isExist" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeIsExistReturns (pub bool) ; impl Schema for ProductFactoryProductTypeIsExistReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ProductFactoryProductTypeIsExistReturns { fn topic () -> & 'static str { "productFactory_productType_isExist" } fn method () -> & 'static str { "productType_isExist" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } }