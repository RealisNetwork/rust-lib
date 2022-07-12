use crate::agents::AgentParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Object {
    properties: HashMap<String, AgentParams>,
    required: Option<Vec<String>>,
}
