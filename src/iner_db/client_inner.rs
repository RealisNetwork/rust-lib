use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio_postgres::{Connection, Socket, Client};
use tokio_postgres::tls::{MakeTlsConnect, TlsStream};
use crate::error_registry::{Db, RealisErrors};

pub struct DatabaseClientInner {
    pub client: Client,
    connection_is_alive: Arc<AtomicBool>,
}

impl DatabaseClientInner {
    pub(crate) async fn new<M: MakeTlsConnect<Socket>>(config: &str, tls: M) -> Result<Self, ()>
        where <M as MakeTlsConnect<Socket>>::Stream: Send + 'static
    {
        let (client, connection) = tokio_postgres::connect(config, tls).await.unwrap(); // TODO handle this unwrap

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
}
