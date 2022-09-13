// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogCategoryCreateParams { # [serde (rename = "name")] pub name : String } impl Schema for BlogCategoryCreateParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"]}") } } impl Agent for BlogCategoryCreateParams { fn topic () -> & 'static str { "blog_category_create" } fn method () -> & 'static str { "category_create" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogCategoryCreateReturns (pub bool) ; impl Schema for BlogCategoryCreateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for BlogCategoryCreateReturns { fn topic () -> & 'static str { "blog_category_create" } fn method () -> & 'static str { "category_create" } fn agent () -> & 'static str { "blog" } }