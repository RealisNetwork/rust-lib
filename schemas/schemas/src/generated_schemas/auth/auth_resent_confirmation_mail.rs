// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthResentConfirmationMailParams { # [serde (rename = "email")] pub email : String } impl Schema for AuthAuthResentConfirmationMailParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"}},\"required\":[\"email\"]}") } } impl Agent for AuthAuthResentConfirmationMailParams { fn topic () -> & 'static str { "auth_auth_resentConfirmationMail" } fn method () -> & 'static str { "auth_resentConfirmationMail" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthResentConfirmationMailReturns (bool) ; impl Schema for AuthAuthResentConfirmationMailReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthResentConfirmationMailReturns { fn topic () -> & 'static str { "auth_auth_resentConfirmationMail" } fn method () -> & 'static str { "auth_resentConfirmationMail" } fn agent () -> & 'static str { "auth" } }