// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketPlaceCancelSaleParams { # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "productId")] pub product_id : i32 } impl Schema for MarketPlaceMarketPlaceCancelSaleParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\",\"userId\"]}") } } impl Agent for MarketPlaceMarketPlaceCancelSaleParams { fn topic () -> & 'static str { "market-place_marketPlace_cancelSale" } fn method () -> & 'static str { "marketPlace_cancelSale" } fn agent () -> & 'static str { "market-place" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketPlaceMarketPlaceCancelSaleReturns (pub bool) ; impl Schema for MarketPlaceMarketPlaceCancelSaleReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for MarketPlaceMarketPlaceCancelSaleReturns { fn topic () -> & 'static str { "market-place_marketPlace_cancelSale" } fn method () -> & 'static str { "marketPlace_cancelSale" } fn agent () -> & 'static str { "market-place" } }