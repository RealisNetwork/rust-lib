// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusMembershipUpdateParams { # [serde (rename = "purchaseToken")] pub purchase_token : String , # [serde (rename = "isActive")] pub is_active : bool } impl Schema for StatusMembershipUpdateParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"purchaseToken\":{\"type\":\"string\"},\"isActive\":{\"type\":\"boolean\"}},\"required\":[\"purchaseToken\",\"isActive\"]}") . unwrap () } } impl Agent for StatusMembershipUpdateParams { fn topic () -> & 'static str { "status_membership_update" } fn method () -> & 'static str { "membership_update" } fn agent () -> & 'static str { "status" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusMembershipUpdateReturns (pub bool) ; impl Schema for StatusMembershipUpdateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for StatusMembershipUpdateReturns { fn topic () -> & 'static str { "status_membership_update" } fn method () -> & 'static str { "membership_update" } fn agent () -> & 'static str { "status" } fn access_level () -> AccessLevel { } }