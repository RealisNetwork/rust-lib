// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetParams { # [serde (rename = "id")] pub id : i64 } impl Schema for BlogBlogGetParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") } } impl Agent for BlogBlogGetParams { fn topic () -> & 'static str { "blog_blog_get" } fn method () -> & 'static str { "blog_get" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetReturnsCategoryParams { # [serde (rename = "name")] pub name : String , # [serde (rename = "isAvailable")] pub is_available : bool , # [serde (rename = "id")] pub id : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetReturns { # [serde (rename = "lang")] pub lang : String , # [serde (rename = "image")] pub image : String , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "category")] pub category : BlogBlogGetReturnsCategoryParams , # [serde (rename = "shortDescription")] pub short_description : String , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "metaDescription")] pub meta_description : String , # [serde (rename = "title")] pub title : String , # [serde (rename = "isAvailable")] pub is_available : bool , # [serde (rename = "metaTitle")] pub meta_title : String , # [serde (rename = "isPinned")] pub is_pinned : bool , # [serde (rename = "url")] pub url : String , # [serde (rename = "views")] pub views : i64 , # [serde (rename = "content")] pub content : String } impl Schema for BlogBlogGetReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lang\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"category\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\",\"isAvailable\"]},\"shortDescription\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"metaTitle\":{\"type\":\"string\"},\"isPinned\":{\"type\":\"boolean\"},\"url\":{\"type\":\"string\"},\"views\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"content\":{\"type\":\"string\"}},\"required\":[\"id\",\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\",\"isPinned\",\"views\",\"isAvailable\",\"lang\",\"createdAt\",\"updatedAt\",\"category\"]}") } } impl Agent for BlogBlogGetReturns { fn topic () -> & 'static str { "blog_blog_get" } fn method () -> & 'static str { "blog_get" } fn agent () -> & 'static str { "blog" } }