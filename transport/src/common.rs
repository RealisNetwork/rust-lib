use error_registry::BaseError;
use serde_json::Value;

pub type TransportResult<T> = Result<T, BaseError<Value>>;
