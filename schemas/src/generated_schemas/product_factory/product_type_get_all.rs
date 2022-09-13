// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeGetAllParams { # [serde (rename = "page")] pub page : Option < f64 > , # [serde (rename = "perPage")] pub per_page : Option < f64 > } impl Schema for ProductFactoryProductTypeGetAllParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":null}") } } impl Agent for ProductFactoryProductTypeGetAllParams { fn topic () -> & 'static str { "productFactory_productType_getAll" } fn method () -> & 'static str { "productType_getAll" } fn agent () -> & 'static str { "productFactory" } } impl < 'de > Deserialize < 'de > for ProductFactoryProductTypeGetAllReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ProductFactoryProductTypeGetAllReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductTypeGetAllReturns ; impl Schema for ProductFactoryProductTypeGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductTypeGetAllReturns { fn topic () -> & 'static str { "productFactory_productType_getAll" } fn method () -> & 'static str { "productType_getAll" } fn agent () -> & 'static str { "productFactory" } }