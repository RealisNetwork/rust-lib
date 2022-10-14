// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct WithdrawApprovalApproveParams { # [serde (rename = "attemptId")] pub attempt_id : f64 } impl Schema for WithdrawApprovalApproveParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"attemptId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"attemptId\"]}") . unwrap () } } impl Agent for WithdrawApprovalApproveParams { fn topic () -> & 'static str { "withdraw_approval_approve" } fn method () -> & 'static str { "approval_approve" } fn agent () -> & 'static str { "withdraw" } fn access_level () -> AccessLevel { AccessLevel :: Private } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct WithdrawApprovalApproveReturns (pub bool) ; impl Schema for WithdrawApprovalApproveReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for WithdrawApprovalApproveReturns { fn topic () -> & 'static str { "withdraw_approval_approve" } fn method () -> & 'static str { "approval_approve" } fn agent () -> & 'static str { "withdraw" } fn access_level () -> AccessLevel { AccessLevel :: Private } }