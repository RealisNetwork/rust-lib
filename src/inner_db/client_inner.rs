use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use log::{error, trace};
use tokio_postgres::{Connection, Socket, Client};
use tokio_postgres::tls::{MakeTlsConnect, TlsStream};
use rawsql::Loader;
use crate::error_registry::{Db, Fs, RealisErrors};

pub struct DatabaseClientInner {
    pub client: Client,
    connection_is_alive: Arc<AtomicBool>,
}

impl DatabaseClientInner {
    pub(crate) async fn new<M: MakeTlsConnect<Socket>>(config: &str, tls: M) -> Result<Self, tokio_postgres::Error>
        where <M as MakeTlsConnect<Socket>>::Stream: Send + 'static
    {
        let (client, connection) = tokio_postgres::connect(config, tls).await?;

        let connection_is_alive = Arc::new(AtomicBool::new(true));

        Self::spawn_connection_handler(connection, connection_is_alive.clone());

        Ok(
            Self {
                client,
                connection_is_alive,
            }
        )
    }

    fn spawn_connection_handler<T>(connection: Connection<Socket, T>, connection_is_alive: Arc<AtomicBool>)
        where T: TlsStream + Unpin + Send + 'static
    {
        tokio::spawn(
            async move {
                if connection.await.is_err() {
                    connection_is_alive.store(false, Ordering::Release);
                }
            }
        );
    }

    pub async fn still_alive(&self) -> Result<(), RealisErrors> {
        if self.connection_is_alive.load(Ordering::Acquire) {
            Err(RealisErrors::Db(Db::Disconnected))
        } else {
            Ok(())
        }
    }

    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), RealisErrors> {
        self.still_alive()?;

        let futures = Loader::get_queries_from(path)
            .map_err(|error| {
                error!("{:?}", error);
                RealisErrors::Fs(Fs::ReadFile)
            })?
            .queries
            .into_iter()
            .sorted()
            .map(|(_, query)| self.client.execute(&query, &[]));

        for future in futures {
            match future.await {
                Ok(_) => trace!("Successful send query!"),
                Err(error) => error!("Cannot send query: {:?}", error),
            }
        }

        Ok(())
    }
}
