// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductMarkHeroAsNftParams { # [serde (rename = "productId")] pub product_id : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for ProductFactoryProductMarkHeroAsNftParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"productId\",\"userId\"]}") . unwrap () } } impl Agent for ProductFactoryProductMarkHeroAsNftParams { fn topic () -> & 'static str { "productFactory_product_markHeroAsNFT" } fn method () -> & 'static str { "product_markHeroAsNFT" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for ProductFactoryProductMarkHeroAsNftReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (ProductFactoryProductMarkHeroAsNftReturns) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductMarkHeroAsNftReturns ; impl Schema for ProductFactoryProductMarkHeroAsNftReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductMarkHeroAsNftReturns { fn topic () -> & 'static str { "productFactory_product_markHeroAsNFT" } fn method () -> & 'static str { "product_markHeroAsNFT" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } }