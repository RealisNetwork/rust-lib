// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetUserByParamsParams { # [serde (rename = "nickname")] pub nickname : String , # [serde (rename = "supportId")] pub support_id : String , # [serde (rename = "lisWallet")] pub lis_wallet : String , # [serde (rename = "email")] pub email : String , # [serde (rename = "ethWallet")] pub eth_wallet : String , # [serde (rename = "GPA")] pub gpa : String } impl Schema for UserProfileGetUserByParamsParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"supportId\":{\"type\":\"string\"},\"lisWallet\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"ethWallet\":{\"type\":\"string\"},\"GPA\":{\"type\":\"string\"}},\"required\":null}") } } impl Agent for UserProfileGetUserByParamsParams { fn topic () -> & 'static str { "user_profile_getUserByParams" } fn method () -> & 'static str { "profile_getUserByParams" } fn agent () -> & 'static str { "user" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetUserByParamsReturnsBanParams { # [serde (rename = "reason")] pub reason : String , # [serde (rename = "whoBanned")] pub who_banned : String , # [serde (rename = "createdAt")] pub created_at : i64 , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "updatedAt")] pub updated_at : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetUserByParamsReturns { # [serde (rename = "isNicknameChanged")] pub is_nickname_changed : bool , # [serde (rename = "isDeleted")] pub is_deleted : bool , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "suspicious")] pub suspicious : bool , # [serde (rename = "verified")] pub verified : bool , # [serde (rename = "ban")] pub ban : UserProfileGetUserByParamsReturnsBanParams , # [serde (rename = "isSubscribedToMailing")] pub is_subscribed_to_mailing : bool , # [serde (rename = "registeredAt")] pub registered_at : i64 , # [serde (rename = "nickname")] pub nickname : String , # [serde (rename = "isConfirmed")] pub is_confirmed : bool , # [serde (rename = "email")] pub email : String , # [serde (rename = "notice")] pub notice : String , # [serde (rename = "isBanned")] pub is_banned : bool } impl Schema for UserProfileGetUserByParamsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isNicknameChanged\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"verified\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"reason\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\"]},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"ban\",\"registeredAt\"]}") } } impl Agent for UserProfileGetUserByParamsReturns { fn topic () -> & 'static str { "user_profile_getUserByParams" } fn method () -> & 'static str { "profile_getUserByParams" } fn agent () -> & 'static str { "user" } }