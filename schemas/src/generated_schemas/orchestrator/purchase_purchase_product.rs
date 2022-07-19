// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorPurchasePurchaseProductParams { # [serde (rename = "creator")] pub creator : String , # [serde (rename = "productType")] pub product_type : String } impl Schema for OrchestratorPurchasePurchaseProductParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"productType\":{\"type\":\"string\"}},\"required\":[\"productType\",\"creator\"]}") } } impl Agent for OrchestratorPurchasePurchaseProductParams { fn topic () -> & 'static str { "orchestrator_purchase_purchaseProduct" } fn method () -> & 'static str { "purchase_purchaseProduct" } fn agent () -> & 'static str { "orchestrator" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorPurchasePurchaseProductReturns (bool) ; impl Schema for OrchestratorPurchasePurchaseProductReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for OrchestratorPurchasePurchaseProductReturns { fn topic () -> & 'static str { "orchestrator_purchase_purchaseProduct" } fn method () -> & 'static str { "purchase_purchaseProduct" } fn agent () -> & 'static str { "orchestrator" } }