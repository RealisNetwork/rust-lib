// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetAllByFilterParams { # [serde (rename = "type")] pub r#type : String , # [serde (rename = "tab")] pub tab : String } impl Schema for AdminOptionGetAllByFilterParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"}},\"required\":null}") } } impl Agent for AdminOptionGetAllByFilterParams { fn topic () -> & 'static str { "admin_option_getAllByFilter" } fn method () -> & 'static str { "option_getAllByFilter" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams { # [serde (rename = "tab")] pub tab : String , # [serde (rename = "type")] pub r#type : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetAllByFilterReturnsParams { # [serde (rename = "previousValue")] pub previous_value : String , # [serde (rename = "value")] pub value : String , # [serde (rename = "description")] pub description : String , # [serde (rename = "scope")] pub scope : String , # [serde (rename = "extraDetails")] pub extra_details : AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams , # [serde (rename = "key")] pub key : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminOptionGetAllByFilterReturns (Vec < AdminOptionGetAllByFilterReturnsParams >) ; impl Schema for AdminOptionGetAllByFilterReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"previousValue\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}},\"required\":null},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\",\"previousValue\",\"description\",\"extraDetails\"]}}") } } impl Agent for AdminOptionGetAllByFilterReturns { fn topic () -> & 'static str { "admin_option_getAllByFilter" } fn method () -> & 'static str { "option_getAllByFilter" } fn agent () -> & 'static str { "admin" } }