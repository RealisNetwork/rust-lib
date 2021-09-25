use serde_json::Value;

pub trait Request {
    type Error;
    /// Create new request object from json
    ///
    /// # Errors
    /// If can't deserialize, return serde_json error
    fn new(json: &Value) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Return unique request id
    /// uuid v4 as String
    fn get_id(&self) -> String;
}

pub trait Gettable {
    /// Return topic by which
    /// getting this request from nats
    fn topic() -> String;
}
