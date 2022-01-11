use openssl::ssl::{SslConnector, SslMethod, SslMode, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tokio_postgres::NoTls;
use crate::healthchecker::HealthChecker;
use crate::inner_db::client_inner::DatabaseClientInner;

pub struct DatabaseClientInnerBuilder;

impl DatabaseClientInnerBuilder {
    pub async fn build_with_params(
        host: &str,
        port: &str,
        user: &str,
        password: &str,
        dbname: &str,
        ssl: bool,
        health: HealthChecker,
    ) -> Result<DatabaseClientInner, tokio_postgres::Error> {
        let general_config = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        );

        if ssl {
            let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();
            builder.set_verify(SslVerifyMode::NONE);
            builder.set_mode(SslMode::AUTO_RETRY);

            let connector = MakeTlsConnector::new(builder.build());

            DatabaseClientInner::new(
                &format!("{} sslmode=require", general_config),
                connector,
                health
            ).await
        } else {
            DatabaseClientInner::new(
                &general_config,
                NoTls,
                health
            ).await
        }
    }
}
