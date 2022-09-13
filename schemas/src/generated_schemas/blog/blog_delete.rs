// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogDeleteParams { # [serde (rename = "id")] pub id : f64 } impl Schema for BlogBlogDeleteParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") } } impl Agent for BlogBlogDeleteParams { fn topic () -> & 'static str { "blog_blog_delete" } fn method () -> & 'static str { "blog_delete" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogDeleteReturns (pub bool) ; impl Schema for BlogBlogDeleteReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for BlogBlogDeleteReturns { fn topic () -> & 'static str { "blog_blog_delete" } fn method () -> & 'static str { "blog_delete" } fn agent () -> & 'static str { "blog" } }