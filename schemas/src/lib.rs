pub mod common;
pub mod generated_schemas;
#[cfg(feature = "manager")]
pub mod manager;

pub use crate::common::*;

use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    Public,
    Protected,
    Private,
    Internal
}

pub trait Schema: DeserializeOwned + Serialize + Clone + Debug + Send + Sync {
    fn schema() -> Value;
}

pub trait Agent: Schema {
    fn topic() -> &'static str;
    fn method() -> &'static str;
    fn agent() -> &'static str;
    fn access_level() -> AccessLevel;
}

impl Schema for Value {
    fn schema() -> Value {
        json!({})
    }
}
