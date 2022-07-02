pub mod common;

pub use crate::common::*;
use serde_json::Value;

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub trait Schema: DeserializeOwned + Serialize + Clone + Debug + Send + Sync {}

impl Schema for Value {}
