// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AdminOptionGetListParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AdminOptionGetListParams) } } # [derive (Debug , Clone , Serialize)] pub struct AdminOptionGetListParams ; impl Schema for AdminOptionGetListParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AdminOptionGetListParams { fn topic () -> & 'static str { "admin_option_getList" } fn method () -> & 'static str { "option_getList" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetListReturnsParams { # [serde (rename = "scope")] pub scope : String , # [serde (rename = "key")] pub key : String , # [serde (rename = "value")] pub value : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetListReturns (Vec < AdminOptionGetListReturnsParams >) ; impl Schema for AdminOptionGetListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}") } } impl Agent for AdminOptionGetListReturns { fn topic () -> & 'static str { "admin_option_getList" } fn method () -> & 'static str { "option_getList" } fn agent () -> & 'static str { "admin" } }