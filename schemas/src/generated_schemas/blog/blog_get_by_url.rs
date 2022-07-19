// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetByUrlParams { # [serde (rename = "url")] pub url : String } impl Schema for BlogBlogGetByUrlParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"url\":{\"type\":\"string\"}},\"required\":[\"url\"]}") } } impl Agent for BlogBlogGetByUrlParams { fn topic () -> & 'static str { "blog_blog_getByUrl" } fn method () -> & 'static str { "blog_getByUrl" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetByUrlReturnsCategoryParams { # [serde (rename = "isAvailable")] pub is_available : bool , # [serde (rename = "name")] pub name : String , # [serde (rename = "id")] pub id : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetByUrlReturns { # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "content")] pub content : String , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "lang")] pub lang : String , # [serde (rename = "url")] pub url : String , # [serde (rename = "shortDescription")] pub short_description : String , # [serde (rename = "image")] pub image : String , # [serde (rename = "title")] pub title : String , # [serde (rename = "metaDescription")] pub meta_description : String , # [serde (rename = "isPinned")] pub is_pinned : bool , # [serde (rename = "metaTitle")] pub meta_title : String , # [serde (rename = "category")] pub category : BlogBlogGetByUrlReturnsCategoryParams , # [serde (rename = "isAvailable")] pub is_available : bool , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "views")] pub views : i64 } impl Schema for BlogBlogGetByUrlReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"lang\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"isPinned\":{\"type\":\"boolean\"},\"metaTitle\":{\"type\":\"string\"},\"category\":{\"type\":\"object\",\"properties\":{\"isAvailable\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\",\"isAvailable\"]},\"isAvailable\":{\"type\":\"boolean\"},\"updatedAt\":{\"type\":\"string\"},\"views\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\",\"isPinned\",\"views\",\"isAvailable\",\"lang\",\"createdAt\",\"updatedAt\",\"category\"]}") } } impl Agent for BlogBlogGetByUrlReturns { fn topic () -> & 'static str { "blog_blog_getByUrl" } fn method () -> & 'static str { "blog_getByUrl" } fn agent () -> & 'static str { "blog" } }