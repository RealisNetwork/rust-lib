// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsRegistryProductAddProductParams { # [serde (rename = "personalType")] pub personal_type : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "productId")] pub product_id : String } impl Schema for TransactionsRegistryProductAddProductParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"productId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productId\",\"personalType\"]}") . unwrap () } } impl Agent for TransactionsRegistryProductAddProductParams { fn topic () -> & 'static str { "transactions_registryProduct_addProduct" } fn method () -> & 'static str { "registryProduct_addProduct" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsRegistryProductAddProductReturns (pub bool) ; impl Schema for TransactionsRegistryProductAddProductReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for TransactionsRegistryProductAddProductReturns { fn topic () -> & 'static str { "transactions_registryProduct_addProduct" } fn method () -> & 'static str { "registryProduct_addProduct" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { } }