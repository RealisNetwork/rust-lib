// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeGetAllByRarityParams { # [serde (rename = "perPage")] pub per_page : Option < i64 > , # [serde (rename = "rarity")] pub rarity : String , # [serde (rename = "page")] pub page : Option < i64 > } impl Schema for ProductFactoryProductTypeGetAllByRarityParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"rarity\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"rarity\"]}") } } impl Agent for ProductFactoryProductTypeGetAllByRarityParams { fn topic () -> & 'static str { "productFactory_productType_getAllByRarity" } fn method () -> & 'static str { "productType_getAllByRarity" } fn agent () -> & 'static str { "productFactory" } } impl < 'de > Deserialize < 'de > for ProductFactoryProductTypeGetAllByRarityReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ProductFactoryProductTypeGetAllByRarityReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductTypeGetAllByRarityReturns ; impl Schema for ProductFactoryProductTypeGetAllByRarityReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductTypeGetAllByRarityReturns { fn topic () -> & 'static str { "productFactory_productType_getAllByRarity" } fn method () -> & 'static str { "productType_getAllByRarity" } fn agent () -> & 'static str { "productFactory" } }