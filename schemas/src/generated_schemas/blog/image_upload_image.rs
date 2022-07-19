// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogImageUploadImageParams { # [serde (rename = "binary")] pub binary : String , # [serde (rename = "extension")] pub extension : String } impl Schema for BlogImageUploadImageParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"binary\":{\"type\":\"string\"},\"extension\":{\"type\":\"string\"}},\"required\":[\"binary\",\"extension\"]}") } } impl Agent for BlogImageUploadImageParams { fn topic () -> & 'static str { "blog_image_uploadImage" } fn method () -> & 'static str { "image_uploadImage" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogImageUploadImageReturns (String) ; impl Schema for BlogImageUploadImageReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for BlogImageUploadImageReturns { fn topic () -> & 'static str { "blog_image_uploadImage" } fn method () -> & 'static str { "image_uploadImage" } fn agent () -> & 'static str { "blog" } }