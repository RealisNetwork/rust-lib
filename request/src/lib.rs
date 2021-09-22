use primitives::Error;
use runtime::Call;
use db::Database;

use serde_json::Value;

pub trait Request {
    fn new(json: Value) -> Result<Self, Error>
        where Self: Sized;

    fn get_id(&self) -> String;
}

pub trait BlockchainRequest {
    fn get_call(&self) -> Result<Call, Error>;
}

pub trait OrchestratorRequest {
    /// Convert request, replace user_id by account_id
    ///
    /// # Errors
    /// Database errors
    fn transform(&self, db: &Database) -> Result<Value, Error>;
}

pub trait Sendable {
    fn get_topic(&self) -> String;

    fn get_json(&self) -> Result<Value, Error>
        where Self: serde::ser::Serialize {
        serde_json::to_value(self).map_err(Error::SerdeJSON)
    }
}
