use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use log::{error, trace};
use tokio_postgres::{Connection, Socket, Client, Error};
use tokio_postgres::tls::{MakeTlsConnect, TlsStream};
use rawsql::Loader;
use itertools::Itertools;
use crate::healthchecker::HealthChecker;

pub struct DatabaseClientInner {
    pub client: Client,
    connection_is_alive: Arc<AtomicBool>,
    health: HealthChecker,
}

impl DatabaseClientInner {
    pub(crate) async fn new<M: MakeTlsConnect<Socket>>(config: &str, tls: M, health: HealthChecker) -> Result<Self, tokio_postgres::Error>
        where <M as MakeTlsConnect<Socket>>::Stream: Send + 'static
    {
        let (client, connection) = tokio_postgres::connect(config, tls).await?;

        let connection_is_alive = Arc::new(AtomicBool::new(true));

        Self::spawn_connection_handler(connection, connection_is_alive.clone(), health.clone());

        Ok(
            Self {
                client,
                connection_is_alive,
                health,
            }
        )
    }

    fn spawn_connection_handler<T>(connection: Connection<Socket, T>, connection_is_alive: Arc<AtomicBool>, health: HealthChecker)
        where T: TlsStream + Unpin + Send + 'static
    {
        tokio::spawn(
            async move {
                if let Err(error) = connection.await {
                    error!("Disconnected from database: {:?}", error);
                    health.make_sick();
                    connection_is_alive.store(false, Ordering::Release);
                }
            }
        );
    }

    pub async fn still_alive(&self) -> Result<(), ()> {
        if self.connection_is_alive.load(Ordering::Acquire) {
            Ok(())
        } else {
            self.health.make_sick();
            Err(())
        }
    }

    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), std::io::Error> {
        let futures = Loader::get_queries_from(path)?
            .queries
            .into_iter()
            .sorted()
            .map(|(_, query)| async move {
                self.client.execute(&query, &[]).await
            });

        for future in futures {
            match future.await {
                Ok(_) => trace!("Successful send query!"),
                Err(error) => error!("Cannot send query: {:?}", error),
            }
        }

        Ok(())
    }
}
