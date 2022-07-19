// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsRegistryProductAddProductParams { # [serde (rename = "productId")] pub product_id : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "personalType")] pub personal_type : String } impl Schema for TransactionsRegistryProductAddProductParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productId\",\"personalType\"]}") } } impl Agent for TransactionsRegistryProductAddProductParams { fn topic () -> & 'static str { "transactions_registryProduct_addProduct" } fn method () -> & 'static str { "registryProduct_addProduct" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsRegistryProductAddProductReturns (bool) ; impl Schema for TransactionsRegistryProductAddProductReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for TransactionsRegistryProductAddProductReturns { fn topic () -> & 'static str { "transactions_registryProduct_addProduct" } fn method () -> & 'static str { "registryProduct_addProduct" } fn agent () -> & 'static str { "transactions" } }