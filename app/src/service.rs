use async_trait::async_trait;
use error_registry::BaseError;
use serde_json::Value;
use transport::response::VResponse;
use schemas::Schema;

#[async_trait]
pub trait Service<T: Schema, G: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> String;

    // TODO: Result<Option<G>, _>
    async fn process(&mut self, request: T) -> Result<G, BaseError<Value>>;
}

//
// impl<S: Schema, D: Schema> From<S> for ServiceResult<S, D> {
//
// }
