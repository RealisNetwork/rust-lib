use crate::env::{Env, EnvLoaded, EnvLoadedError};

#[derive(Debug, Clone, Env)]
pub struct Nats {
    pub host: String,
    pub port: String,
    #[env(flatten)]
    pub client_id: String,
    #[env(flatten)]
    pub cluster_id: String,
    pub service_group: String,
}

impl Nats {
    pub async fn build(&self) -> transport::nats::Nats {
        let nats_options = format!("{}:{}", self.host, self.port);
        transport::nats::Nats::new(&nats_options, self.client_id.as_str(), self.cluster_id.as_str()).await
    }
}
