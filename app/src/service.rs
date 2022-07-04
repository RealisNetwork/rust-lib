use async_trait::async_trait;
use error_registry::BaseError;
use serde_json::Value;
use transport::response::VResponse;
use schemas::Schema;

#[async_trait]
pub trait Service<P: Agent, G: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> &'static str {
        P::topic()
    }

    // TODO: Result<Option<G>, _>
    async fn process(&mut self, request: T) -> Result<G, BaseError<Value>>;
}
