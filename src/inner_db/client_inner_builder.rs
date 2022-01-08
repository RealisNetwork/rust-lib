use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tokio_postgres::NoTls;
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
    ) -> Result<DatabaseClientInner, ()> {
        let general_config = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        );

        if ssl {
            let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
            builder.set_verify(SslVerifyMode::NONE);
            let connector = MakeTlsConnector::new(builder.build());

            DatabaseClientInner::new(
                &format!("{} sslmode=require", general_config),
                connector
            ).await
        } else {
            DatabaseClientInner::new(
                &general_config,
                NoTls
            ).await
        }
    }
}
