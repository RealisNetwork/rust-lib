pub mod common;
pub mod generated_schemas;

pub use crate::common::*;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Debug;

pub trait Schema: DeserializeOwned + Serialize + Clone + Debug + Send + Sync {
    fn schema() -> Value;
}

pub trait Agent: Schema {
    fn topic() -> &'static str;
    fn method() -> &'static str;
    fn agent() -> &'static str;
}

impl Schema for Value {
    fn schema() -> Value {
        json!({})
    }
}
