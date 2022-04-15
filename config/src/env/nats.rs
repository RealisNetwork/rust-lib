use crate::{
    env::{Env, EnvLoaded, EnvLoadedError},
    transport::nats,
};
use rust_lib::healthchecker::HealthChecker;

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
    pub async fn build(&self, healthchecker: HealthChecker) -> Result<nats::Nats, String> {
        let nats_options = format!("{}:{}", self.host, self.port);
        let nats = nats::Nats::new(&nats_options, self.client_id.as_str(), self.cluster_id.as_str()).await;
        // Add disconnect handler - make healthchecker sick if disconnect
        match nats
            .stan_client
            .nats_client
            .add_disconnect_handler(Box::new({
                let health_checker = healthchecker;
                move |_nats_client| {
                    health_checker.make_sick();
                }
            }))
            .await
        {
            Ok(nats_) => Ok(nats_),
            Err(err) => return Err(err.to_string()),
        }
    }
}
