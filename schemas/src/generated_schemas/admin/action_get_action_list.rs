// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AdminActionGetActionListParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AdminActionGetActionListParams) } } # [derive (Debug , Clone , Serialize)] pub struct AdminActionGetActionListParams ; impl Schema for AdminActionGetActionListParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AdminActionGetActionListParams { fn topic () -> & 'static str { "admin_action_getActionList" } fn method () -> & 'static str { "action_getActionList" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminActionGetActionListReturns (pub Vec < String >) ; impl Schema for AdminActionGetActionListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}") } } impl Agent for AdminActionGetActionListReturns { fn topic () -> & 'static str { "admin_action_getActionList" } fn method () -> & 'static str { "action_getActionList" } fn agent () -> & 'static str { "admin" } }