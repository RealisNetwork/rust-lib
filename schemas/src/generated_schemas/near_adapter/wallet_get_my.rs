// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for NearAdapterWalletGetMyParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (NearAdapterWalletGetMyParams) } } # [derive (Debug , Clone , Serialize)] pub struct NearAdapterWalletGetMyParams ; impl Schema for NearAdapterWalletGetMyParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for NearAdapterWalletGetMyParams { fn topic () -> & 'static str { "near-adapter_wallet_getMy" } fn method () -> & 'static str { "wallet_getMy" } fn agent () -> & 'static str { "near-adapter" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct NearAdapterWalletGetMyReturns (pub String) ; impl Schema for NearAdapterWalletGetMyReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for NearAdapterWalletGetMyReturns { fn topic () -> & 'static str { "near-adapter_wallet_getMy" } fn method () -> & 'static str { "wallet_getMy" } fn agent () -> & 'static str { "near-adapter" } }