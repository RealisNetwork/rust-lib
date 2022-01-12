use std::time::Duration;
use deadpool_postgres::{Config, CreatePoolError, ManagerConfig, RecyclingMethod, Runtime, SslMode};
use openssl::error::ErrorStack;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tokio_postgres::{Error, NoTls};
use crate::inner_db::client_inner::DatabaseClientInner;

pub struct DatabaseClientInnerBuilder;

impl DatabaseClientInnerBuilder {
    pub async fn build_with_params(
        host: String,
        port: u16,
        user: String,
        password: String,
        dbname: String,
        keepalives_idle: Option<Duration>,
        ssl: bool,
    ) -> Result<DatabaseClientInner, BuildError> {
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

        let pool = if ssl {
            cfg.ssl_mode = Some(SslMode::Require);

            let mut builder = SslConnector::builder(SslMethod::tls_client())?; // TODO handle this unwrap
            builder.set_verify(SslVerifyMode::NONE);
            builder.set_mode(openssl::ssl::SslMode::AUTO_RETRY);
            let tls = MakeTlsConnector::new(builder.build());

            cfg.create_pool(Some(Runtime::Tokio1), tls)?
        } else {
            cfg.create_pool(Some(Runtime::Tokio1), NoTls)?
        };

        Ok(DatabaseClientInner::new(pool))
    }
}

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
