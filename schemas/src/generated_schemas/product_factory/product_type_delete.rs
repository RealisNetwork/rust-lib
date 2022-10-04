// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeDeleteParams { # [serde (rename = "id")] pub id : f64 } impl Schema for ProductFactoryProductTypeDeleteParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap () } } impl Agent for ProductFactoryProductTypeDeleteParams { fn topic () -> & 'static str { "productFactory_productType_delete" } fn method () -> & 'static str { "productType_delete" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for ProductFactoryProductTypeDeleteReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (ProductFactoryProductTypeDeleteReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductTypeDeleteReturns ; impl Schema for ProductFactoryProductTypeDeleteReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductTypeDeleteReturns { fn topic () -> & 'static str { "productFactory_productType_delete" } fn method () -> & 'static str { "productType_delete" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } }