use crate::{
    client_inner::DatabaseClientInner,
    consts::{MAX_RETRY_ELAPSED_TIME_IN_SECS, MAX_RETRY_INTERVAL_IN_SECS},
};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime, SslMode};
use openssl::{
    ssl::{SslConnector, SslMethod, SslVerifyMode},
};
use postgres_openssl::MakeTlsConnector;
use std::time::Duration;
use tokio_postgres::NoTls;
use error_registry::BaseError;

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
    ) -> Result<DatabaseClientInner, BaseError<()>> {
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
    ) -> Result<DatabaseClientInner, BaseError<()>> {
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
    ) -> Result<Pool, BaseError<()>> {
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
