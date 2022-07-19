// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileDeleteUserParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for UserProfileDeleteUserParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") } } impl Agent for UserProfileDeleteUserParams { fn topic () -> & 'static str { "user_profile_deleteUser" } fn method () -> & 'static str { "profile_deleteUser" } fn agent () -> & 'static str { "user" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileDeleteUserReturns (bool) ; impl Schema for UserProfileDeleteUserReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for UserProfileDeleteUserReturns { fn topic () -> & 'static str { "user_profile_deleteUser" } fn method () -> & 'static str { "profile_deleteUser" } fn agent () -> & 'static str { "user" } }