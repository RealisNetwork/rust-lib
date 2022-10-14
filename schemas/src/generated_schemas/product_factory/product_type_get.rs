// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeGetParams { # [serde (rename = "id")] pub id : f64 } impl Schema for ProductFactoryProductTypeGetParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap () } } impl Agent for ProductFactoryProductTypeGetParams { fn topic () -> & 'static str { "productFactory_productType_get" } fn method () -> & 'static str { "productType_get" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { AccessLevel :: Private } } impl < 'de > Deserialize < 'de > for ProductFactoryProductTypeGetReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (ProductFactoryProductTypeGetReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductTypeGetReturns ; impl Schema for ProductFactoryProductTypeGetReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductTypeGetReturns { fn topic () -> & 'static str { "productFactory_productType_get" } fn method () -> & 'static str { "productType_get" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { AccessLevel :: Private } }