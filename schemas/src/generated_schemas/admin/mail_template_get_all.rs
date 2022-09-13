// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AdminMailTemplateGetAllParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AdminMailTemplateGetAllParams) } } # [derive (Debug , Clone , Serialize)] pub struct AdminMailTemplateGetAllParams ; impl Schema for AdminMailTemplateGetAllParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AdminMailTemplateGetAllParams { fn topic () -> & 'static str { "admin_mailTemplate_getAll" } fn method () -> & 'static str { "mailTemplate_getAll" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminMailTemplateGetAllReturnsParams { # [serde (rename = "mailTemplate")] pub mail_template : String , # [serde (rename = "key")] pub key : String , # [serde (rename = "name")] pub name : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminMailTemplateGetAllReturns (pub Vec < AdminMailTemplateGetAllReturnsParams >) ; impl Schema for AdminMailTemplateGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"mailTemplate\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"key\",\"name\",\"mailTemplate\"]}}") } } impl Agent for AdminMailTemplateGetAllReturns { fn topic () -> & 'static str { "admin_mailTemplate_getAll" } fn method () -> & 'static str { "mailTemplate_getAll" } fn agent () -> & 'static str { "admin" } }