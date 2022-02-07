use crate::env::{Env, EnvLoaded, EnvLoadedError};
use rust_lib::inner_db::{
    client_inner::DatabaseClientInner,
    client_inner_builder::{BuildError, DatabaseClientInnerBuilder},
    consts::KEEPALIVES_IDLE_IN_SECS,
};
use std::time::Duration;

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
