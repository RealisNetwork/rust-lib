use async_trait::async_trait;
use error_registry::BaseError;
use serde_json::Value;
use transport::response::VResponse;

#[async_trait]
pub trait Service<T> {
    fn topic_to_subscribe(&self) -> String;

    async fn process(&mut self, schema: T) -> Result<Vec<VResponse>, BaseError<Value>>;
}
