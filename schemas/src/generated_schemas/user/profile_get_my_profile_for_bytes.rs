// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for UserProfileGetMyProfileForBytesParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (UserProfileGetMyProfileForBytesParams) } } # [derive (Debug , Clone , Serialize)] pub struct UserProfileGetMyProfileForBytesParams ; impl Schema for UserProfileGetMyProfileForBytesParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for UserProfileGetMyProfileForBytesParams { fn topic () -> & 'static str { "user_profile_getMyProfileForBytes" } fn method () -> & 'static str { "profile_getMyProfileForBytes" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetMyProfileForBytesReturns { # [serde (rename = "registeredAt" , deserialize_with = "deserialize_to_string")] pub registered_at : String , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "id")] pub id : i32 , # [serde (rename = "nickname" , deserialize_with = "deserialize_to_string")] pub nickname : String , # [serde (rename = "verified")] pub verified : bool , # [serde (rename = "suspicious")] pub suspicious : bool , # [serde (rename = "isConfirmed")] pub is_confirmed : bool , # [serde (rename = "email" , deserialize_with = "deserialize_to_string")] pub email : String , # [serde (rename = "isNicknameChanged")] pub is_nickname_changed : bool , # [serde (rename = "isDeleted")] pub is_deleted : bool , # [serde (rename = "isSubscribedToMailing")] pub is_subscribed_to_mailing : bool , # [serde (rename = "notice" , deserialize_with = "deserialize_to_string")] pub notice : String , # [serde (rename = "isBanned")] pub is_banned : bool , # [serde (rename = "reason" , deserialize_with = "deserialize_to_string")] pub reason : String } impl Schema for UserProfileGetMyProfileForBytesReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"registeredAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"nickname\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"suspicious\":{\"type\":\"boolean\"},\"isConfirmed\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"notice\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"reason\",\"registeredAt\"]}") } } impl Agent for UserProfileGetMyProfileForBytesReturns { fn topic () -> & 'static str { "user_profile_getMyProfileForBytes" } fn method () -> & 'static str { "profile_getMyProfileForBytes" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }