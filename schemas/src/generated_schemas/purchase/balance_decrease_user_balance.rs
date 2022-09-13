// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PurchaseBalanceDecreaseUserBalanceParams { # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "amount")] pub amount : String , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "currency")] pub currency : String , # [serde (rename = "topicToSuccessResponse")] pub topic_to_success_response : String } impl Schema for PurchaseBalanceDecreaseUserBalanceParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"topicToSuccessResponse\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}") } } impl Agent for PurchaseBalanceDecreaseUserBalanceParams { fn topic () -> & 'static str { "purchase_balance_decreaseUserBalance" } fn method () -> & 'static str { "balance_decreaseUserBalance" } fn agent () -> & 'static str { "purchase" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PurchaseBalanceDecreaseUserBalanceReturns (pub bool) ; impl Schema for PurchaseBalanceDecreaseUserBalanceReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for PurchaseBalanceDecreaseUserBalanceReturns { fn topic () -> & 'static str { "purchase_balance_decreaseUserBalance" } fn method () -> & 'static str { "balance_decreaseUserBalance" } fn agent () -> & 'static str { "purchase" } }