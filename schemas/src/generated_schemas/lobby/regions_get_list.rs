// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for LobbyRegionsGetListParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (LobbyRegionsGetListParams) } } # [derive (Debug , Clone , Serialize)] pub struct LobbyRegionsGetListParams ; impl Schema for LobbyRegionsGetListParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for LobbyRegionsGetListParams { fn topic () -> & 'static str { "lobby_regions_getList" } fn method () -> & 'static str { "regions_getList" } fn agent () -> & 'static str { "lobby" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyRegionsGetListReturnsListParamsParams { # [serde (rename = "host")] pub host : String , # [serde (rename = "code")] pub code : String , # [serde (rename = "name")] pub name : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyRegionsGetListReturns { # [serde (rename = "list")] pub list : Vec < LobbyRegionsGetListReturnsListParamsParams > } impl Schema for LobbyRegionsGetListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"list\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"host\":{\"type\":\"string\"},\"code\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"host\",\"code\"]}}},\"required\":[\"list\"]}") } } impl Agent for LobbyRegionsGetListReturns { fn topic () -> & 'static str { "lobby_regions_getList" } fn method () -> & 'static str { "regions_getList" } fn agent () -> & 'static str { "lobby" } }