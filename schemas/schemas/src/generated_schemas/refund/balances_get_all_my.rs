// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for RefundBalancesGetAllMyParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (RefundBalancesGetAllMyParams) } } # [derive (Debug , Clone , Serialize)] pub struct RefundBalancesGetAllMyParams ; impl Schema for RefundBalancesGetAllMyParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for RefundBalancesGetAllMyParams { fn topic () -> & 'static str { "refund_balances_getAllMy" } fn method () -> & 'static str { "balances_getAllMy" } fn agent () -> & 'static str { "refund" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RefundBalancesGetAllMyReturns { # [serde (rename = "LIS")] pub lis : String , # [serde (rename = "ETH")] pub eth : String } impl Schema for RefundBalancesGetAllMyReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"LIS\":{\"type\":\"string\"},\"ETH\":{\"type\":\"string\"}},\"required\":null}") } } impl Agent for RefundBalancesGetAllMyReturns { fn topic () -> & 'static str { "refund_balances_getAllMy" } fn method () -> & 'static str { "balances_getAllMy" } fn agent () -> & 'static str { "refund" } }