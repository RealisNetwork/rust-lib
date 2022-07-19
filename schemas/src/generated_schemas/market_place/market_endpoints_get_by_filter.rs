// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams { # [serde (rename = "column")] pub column : String , # [serde (rename = "value")] pub value : () } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterParamsOrderByParams { # [serde (rename = "desc")] pub desc : bool , # [serde (rename = "column")] pub column : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterParams { # [serde (rename = "filters")] pub filters : Option < Vec < MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams > > , # [serde (rename = "orderBy")] pub order_by : Option < MarketPlaceMarketEndpointsGetByFilterParamsOrderByParams > , # [serde (rename = "perPage")] pub per_page : i32 , # [serde (rename = "page")] pub page : i32 , # [serde (rename = "category")] pub category : String } impl Schema for MarketPlaceMarketEndpointsGetByFilterParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"filters\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"column\":{\"type\":\"string\"},\"value\":{}},\"required\":[\"column\",\"value\"]}},\"orderBy\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"column\":{\"type\":\"string\"}},\"required\":[\"column\",\"desc\"]},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"string\"}},\"required\":[\"category\",\"page\",\"perPage\"]}") } } impl Agent for MarketPlaceMarketEndpointsGetByFilterParams { fn topic () -> & 'static str { "market-place_marketEndpoints_getByFilter" } fn method () -> & 'static str { "marketEndpoints_getByFilter" } fn agent () -> & 'static str { "market-place" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParams { # [serde (rename = "productId")] pub product_id : i32 , # [serde (rename = "type")] pub r#type : String , # [serde (rename = "subType")] pub sub_type : String , # [serde (rename = "createdAt")] pub created_at : i32 , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "additionalParams")] pub additional_params : MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams , # [serde (rename = "price")] pub price : i32 , # [serde (rename = "personalType")] pub personal_type : String , # [serde (rename = "category")] pub category : i32 , # [serde (rename = "isLocked")] pub is_locked : bool , # [serde (rename = "id")] pub id : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketEndpointsGetByFilterReturns (Vec < MarketPlaceMarketEndpointsGetByFilterReturnsParams >) ; impl Schema for MarketPlaceMarketEndpointsGetByFilterReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isLocked\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\"]}}") } } impl Agent for MarketPlaceMarketEndpointsGetByFilterReturns { fn topic () -> & 'static str { "market-place_marketEndpoints_getByFilter" } fn method () -> & 'static str { "marketEndpoints_getByFilter" } fn agent () -> & 'static str { "market-place" } }