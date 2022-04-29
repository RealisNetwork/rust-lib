use crate::env::{Env, EnvLoaded, EnvLoadedError};
use rust_lib::healthchecker::HealthChecker;

#[derive(Debug, Clone, Env)]
pub struct Nats_v2 {
    pub host: String,
    pub port: String,
    #[env(flatten)]
    pub client_id: String,
    #[env(flatten)]
    pub cluster_id: String,
    pub service_group: String,
}

impl Nats_v2 {

    pub async fn build(&self, healthchecker: HealthChecker) -> Result<transport::nats_v2::Nats_v2, String> {
        let nats_options = format!("{}:{}", self.host, self.port);
        let nats = match transport::nats_v2::Nats_v2::new(&nats_options, self.client_id.as_str()) {
            Ok(nats_) => nats_,
            Err(err) => return Err(err.to_string()),
        };

        Ok(nats)
    }
}
