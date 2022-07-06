use async_trait::async_trait;
use error_registry::BaseError;
use schemas::{common::Request, Agent, Schema};
use serde_json::Value;

#[async_trait]
pub trait Service<P: Agent, G: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> &'static str {
        P::topic()
    }

    async fn process(&mut self, request: Request<P>) -> Result<G, BaseError<Value>>;
}
