// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeGetAllByTypeParams { # [serde (rename = "type")] pub r#type : String , # [serde (rename = "perPage")] pub per_page : Option < f64 > , # [serde (rename = "page")] pub page : Option < f64 > } impl Schema for ProductFactoryProductTypeGetAllByTypeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"type\"]}") . unwrap () } } impl Agent for ProductFactoryProductTypeGetAllByTypeParams { fn topic () -> & 'static str { "productFactory_productType_getAllByType" } fn method () -> & 'static str { "productType_getAllByType" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for ProductFactoryProductTypeGetAllByTypeReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (ProductFactoryProductTypeGetAllByTypeReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductTypeGetAllByTypeReturns ; impl Schema for ProductFactoryProductTypeGetAllByTypeReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductTypeGetAllByTypeReturns { fn topic () -> & 'static str { "productFactory_productType_getAllByType" } fn method () -> & 'static str { "productType_getAllByType" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } }