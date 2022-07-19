// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetAllByFilterParams { # [serde (rename = "limit")] pub limit : i64 , # [serde (rename = "perPage")] pub per_page : i64 , # [serde (rename = "categoryId")] pub category_id : i64 , # [serde (rename = "articleId")] pub article_id : i64 , # [serde (rename = "page")] pub page : i64 } impl Schema for BlogBlogGetAllByFilterParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"limit\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"categoryId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"articleId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":null}") } } impl Agent for BlogBlogGetAllByFilterParams { fn topic () -> & 'static str { "blog_blog_getAllByFilter" } fn method () -> & 'static str { "blog_getAllByFilter" } fn agent () -> & 'static str { "blog" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetAllByFilterReturnsDataParamsParams { # [serde (rename = "title")] pub title : String , # [serde (rename = "image")] pub image : String , # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "url")] pub url : String , # [serde (rename = "shortDescription")] pub short_description : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogBlogGetAllByFilterReturns { # [serde (rename = "data")] pub data : Vec < BlogBlogGetAllByFilterReturnsDataParamsParams > , # [serde (rename = "totalCount")] pub total_count : i64 } impl Schema for BlogBlogGetAllByFilterReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"}},\"required\":[\"title\",\"image\",\"url\",\"createdAt\",\"shortDescription\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}") } } impl Agent for BlogBlogGetAllByFilterReturns { fn topic () -> & 'static str { "blog_blog_getAllByFilter" } fn method () -> & 'static str { "blog_getAllByFilter" } fn agent () -> & 'static str { "blog" } }