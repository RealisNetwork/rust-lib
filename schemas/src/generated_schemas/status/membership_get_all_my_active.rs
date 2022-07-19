// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for StatusMembershipGetAllMyActiveParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (StatusMembershipGetAllMyActiveParams) } } # [derive (Debug , Clone , Serialize)] pub struct StatusMembershipGetAllMyActiveParams ; impl Schema for StatusMembershipGetAllMyActiveParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for StatusMembershipGetAllMyActiveParams { fn topic () -> & 'static str { "status_membership_getAllMyActive" } fn method () -> & 'static str { "membership_getAllMyActive" } fn agent () -> & 'static str { "status" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusMembershipGetAllMyActiveReturnsParamsMembershipParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusMembershipGetAllMyActiveReturnsParams { # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "endDate")] pub end_date : String , # [serde (rename = "isActive")] pub is_active : bool , # [serde (rename = "duration")] pub duration : String , # [serde (rename = "appId")] pub app_id : i64 , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "subscriptionOrderId")] pub subscription_order_id : String , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "subscriptionProductId")] pub subscription_product_id : String , # [serde (rename = "membership")] pub membership : StatusMembershipGetAllMyActiveReturnsParamsMembershipParams , # [serde (rename = "userId")] pub user_id : i64 , # [serde (rename = "subscriptionTokenHash")] pub subscription_token_hash : String , # [serde (rename = "subscriptionToken")] pub subscription_token : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusMembershipGetAllMyActiveReturns (Vec < StatusMembershipGetAllMyActiveReturnsParams >) ; impl Schema for StatusMembershipGetAllMyActiveReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"endDate\":{\"type\":\"string\"},\"isActive\":{\"type\":\"boolean\"},\"duration\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"subscriptionOrderId\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"subscriptionProductId\":{\"type\":\"string\"},\"membership\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"userId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"subscriptionTokenHash\":{\"type\":\"string\"},\"subscriptionToken\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"appId\",\"isActive\",\"endDate\",\"duration\",\"subscriptionProductId\",\"subscriptionOrderId\",\"subscriptionToken\",\"subscriptionTokenHash\",\"createdAt\",\"updatedAt\",\"membership\"]}}") } } impl Agent for StatusMembershipGetAllMyActiveReturns { fn topic () -> & 'static str { "status_membership_getAllMyActive" } fn method () -> & 'static str { "membership_getAllMyActive" } fn agent () -> & 'static str { "status" } }