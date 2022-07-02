use async_trait::async_trait;
use error_registry::BaseError;
use serde_json::Value;
use transport::response::VResponse;
use schemas::Schema;

#[async_trait]
pub trait Service<T: Schema, G: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> String;

    // TODO: remove vec
    async fn process(&mut self, request: T) -> Result<Vec<ServiceResult<G>>, BaseError<Value>>;
}

pub enum ServiceResult<T: Schema> {
    RawResult(T),
    A(T),
    // OrchestratorResult(&'a dyn Schema, String),
    // NotificationResult(&'a dyn Schema, &'a dyn Schema, String)
}
//
// impl<S: Schema, D: Schema> From<S> for ServiceResult<S, D> {
//
// }
