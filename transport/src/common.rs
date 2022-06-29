use serde_json::Value;
use error_registry::BaseError;

pub type TransportResult<T> = Result<T, BaseError<Value>>;