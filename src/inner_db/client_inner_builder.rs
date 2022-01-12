use deadpool_postgres::{Config, PoolConfig, Runtime, SslMode};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tokio_postgres::NoTls;
use crate::healthchecker::HealthChecker;
use crate::inner_db::client_inner::DatabaseClientInner;

pub struct DatabaseClientInnerBuilder;

impl DatabaseClientInnerBuilder {
    pub async fn build_with_params(
        host: &str,
        port: &str, // TODO pass port as u16
        user: &str,
        password: &str,
        dbname: &str,
        ssl: bool,
    ) -> Result<DatabaseClientInner, tokio_postgres::Error> {
        let mut cfg = Config::new();
        cfg.host = Some(host.to_string());
        cfg.port = Some(port.parse::<u16>().unwrap());
        cfg.user = Some(user.to_string());
        cfg.password = Some(password.to_string());
        cfg.dbname = Some(dbname.to_string());

        let pool = if ssl {
            cfg.ssl_mode = Some(SslMode::Require);

            let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();
            builder.set_verify(SslVerifyMode::NONE);
            builder.set_mode(openssl::ssl::SslMode::AUTO_RETRY);
            cfg.create_pool(Some(Runtime::Tokio1), MakeTlsConnector::new(builder.build())) // TODO handle this unwrap
        } else {
            cfg.create_pool(Some(Runtime::Tokio1), NoTls) // TODO handle this unwrap
        }.unwrap();

        DatabaseClientInner::new(pool, health)
    }
}
