// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsBalancerBalancerServerStoppedParams { # [serde (rename = "clientId" , deserialize_with = "deserialize_to_string")] pub client_id : String } impl Schema for DragocatsBalancerBalancerServerStoppedParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientId\":{\"type\":\"string\"}},\"required\":[\"clientId\"]}") . unwrap () } } impl Agent for DragocatsBalancerBalancerServerStoppedParams { fn topic () -> & 'static str { "dragocats-balancer_balancer_serverStopped" } fn method () -> & 'static str { "balancer_serverStopped" } fn agent () -> & 'static str { "dragocats-balancer" } fn access_level () -> AccessLevel { AccessLevel :: Internal } } impl < 'de > Deserialize < 'de > for DragocatsBalancerBalancerServerStoppedReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragocatsBalancerBalancerServerStoppedReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragocatsBalancerBalancerServerStoppedReturns ; impl Schema for DragocatsBalancerBalancerServerStoppedReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragocatsBalancerBalancerServerStoppedReturns { fn topic () -> & 'static str { "dragocats-balancer_balancer_serverStopped" } fn method () -> & 'static str { "balancer_serverStopped" } fn agent () -> & 'static str { "dragocats-balancer" } fn access_level () -> AccessLevel { AccessLevel :: Internal } }