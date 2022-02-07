use std::time::Duration;
use rust_lib::inner_db::client_inner::DatabaseClientInner;
use rust_lib::inner_db::client_inner_builder::{BuildError, DatabaseClientInnerBuilder};
use rust_lib::inner_db::consts::KEEPALIVES_IDLE_IN_SECS;
use crate::env::{EnvLoadedError, Env, EnvLoaded};

#[derive(Debug, Clone, Env)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub ssl: bool,
}

impl Database {
    pub async fn build(&self) -> Result<DatabaseClientInner, BuildError> {
        DatabaseClientInnerBuilder::build_with_params(
            self.host.clone(),
            self.port,
            self.user.clone(),
            self.password.clone(),
            self.name.clone(),
            Some(Duration::from_secs(KEEPALIVES_IDLE_IN_SECS)),
            self.ssl,
        )
            .await
    }
}
