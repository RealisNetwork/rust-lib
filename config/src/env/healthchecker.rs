use crate::env::env_loaded::{Env, EnvLoaded};
use error_registry::BaseError;
use healthchecker::HealthcheckerServer;

#[derive(Debug, Clone, Env)]
pub struct Healthchecker {
    #[env(rename_abs = "HEALTHCHECK")]
    pub host: String,
}

impl Healthchecker {
    pub async fn build(&self) -> Result<HealthcheckerServer, BaseError<()>> {
        HealthcheckerServer::new(None, &self.host).await
    }
}
