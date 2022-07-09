pub mod common;

pub use crate::common::*;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait Schema: DeserializeOwned + Serialize + Clone + Debug + Send + Sync {}

pub trait Agent: Schema {
    fn topic() -> &'static str;
    fn method() -> &'static str;
    fn agent() -> &'static str;
}
