use crate::env::{Env, EnvLoaded, EnvLoadedError};
use healthchecker::HealthChecker;

#[derive(Debug, Clone, Env)]
pub struct Healthchecker {
    #[env(rename_abs = "HEALTHCHECK")]
    pub host: String,
}

impl Healthchecker {
    pub async fn build(&self) -> Result<HealthChecker, String> {
        HealthChecker::new(&self.host, 10000).await
    }
}
