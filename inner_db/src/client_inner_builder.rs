use crate::{
    client_inner::DatabaseClientInner,
    consts::{MAX_RETRY_ELAPSED_TIME_IN_SECS, MAX_RETRY_INTERVAL_IN_SECS},
};
use deadpool_postgres::{Config, CreatePoolError, ManagerConfig, Pool, RecyclingMethod, Runtime, SslMode};
use openssl::{
    error::ErrorStack,
    ssl::{SslConnector, SslMethod, SslVerifyMode},
};
use postgres_openssl::MakeTlsConnector;
use std::time::Duration;
use tokio_postgres::{Error, NoTls};

pub struct DatabaseClientInnerBuilder;

impl DatabaseClientInnerBuilder {
    pub async fn build_with_retry_params(
        host: String,
        port: u16,
        user: String,
        password: String,
        dbname: String,
        keepalives_idle: Option<Duration>,
        ssl: bool,
        max_interval: u64,
        max_elapsed_time: u64,
    ) -> Result<DatabaseClientInner, BuildError> {
        Ok(DatabaseClientInner::new(
            Self::create_pool(host, port, user, password, dbname, keepalives_idle, ssl)?,
            max_interval,
            max_elapsed_time,
        ))
    }

    pub async fn build_with_params(
        host: String,
        port: u16,
        user: String,
        password: String,
        dbname: String,
        keepalives_idle: Option<Duration>,
        ssl: bool,
    ) -> Result<DatabaseClientInner, BuildError> {
        Ok(DatabaseClientInner::new(
            Self::create_pool(host, port, user, password, dbname, keepalives_idle, ssl)?,
            MAX_RETRY_INTERVAL_IN_SECS,
            MAX_RETRY_ELAPSED_TIME_IN_SECS,
        ))
    }

    fn create_pool(
        host: String,
        port: u16,
        user: String,
        password: String,
        dbname: String,
        keepalives_idle: Option<Duration>,
        ssl: bool,
    ) -> Result<Pool, BuildError> {
        let mut cfg = Config::new();
        cfg.host = Some(host);
        cfg.port = Some(port);
        cfg.user = Some(user);
        cfg.password = Some(password);
        cfg.dbname = Some(dbname);
        cfg.keepalives_idle = keepalives_idle;
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Verified,
        });

        if ssl {
            cfg.ssl_mode = Some(SslMode::Require);

            let mut builder = SslConnector::builder(SslMethod::tls_client())?; // TODO handle this unwrap
            builder.set_verify(SslVerifyMode::NONE);
            builder.set_mode(openssl::ssl::SslMode::AUTO_RETRY);
            let tls = MakeTlsConnector::new(builder.build());

            Ok(cfg.create_pool(Some(Runtime::Tokio1), tls)?)
        } else {
            Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
        }
    }
}

#[derive(Debug)]
pub enum BuildError {
    Postgres(tokio_postgres::Error),
    Ssl(openssl::error::ErrorStack),
    Pool(deadpool_postgres::CreatePoolError),
}

impl From<tokio_postgres::Error> for BuildError {
    fn from(error: Error) -> Self {
        BuildError::Postgres(error)
    }
}

impl From<ErrorStack> for BuildError {
    fn from(error: ErrorStack) -> Self {
        BuildError::Ssl(error)
    }
}

impl From<CreatePoolError> for BuildError {
    fn from(error: CreatePoolError) -> Self {
        BuildError::Pool(error)
    }
}