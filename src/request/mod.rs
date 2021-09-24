use crate::db::Database;

use crate::primitives::adapter::Error;
use serde_json::Value;

pub trait Request {
    /// Create new request object from json
    ///
    /// # Errors
    /// If can't deserialize, return serde_json error
    fn new(json: &Value) -> Result<Self, Error>
    where
        Self: Sized;

    /// Return unique request id
    /// uuid v4 as String
    fn get_id(&self) -> String;
}

pub trait OrchestratorRequest {
    /// Return topic by which publish
    /// response for this request into nats
    fn get_topic(&self) -> String;

    /// Convert request, replace user_id by account_id
    ///
    /// # Errors
    /// Database errors
    fn process(&self, db: &Database) -> Result<Value, Error>;
}

pub trait Gettable {
    /// Return topic by which
    /// getting this request from nats
    fn topic() -> String;
}
