// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorLootboxPurchaseParams { # [serde (rename = "personalType")] pub personal_type : String } impl Schema for OrchestratorLootboxPurchaseParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap () } } impl Agent for OrchestratorLootboxPurchaseParams { fn topic () -> & 'static str { "orchestrator_lootbox_purchase" } fn method () -> & 'static str { "lootbox_purchase" } fn agent () -> & 'static str { "orchestrator" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorLootboxPurchaseReturns (pub bool) ; impl Schema for OrchestratorLootboxPurchaseReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for OrchestratorLootboxPurchaseReturns { fn topic () -> & 'static str { "orchestrator_lootbox_purchase" } fn method () -> & 'static str { "lootbox_purchase" } fn agent () -> & 'static str { "orchestrator" } fn access_level () -> AccessLevel { } }