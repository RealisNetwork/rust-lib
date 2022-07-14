use async_trait::async_trait;
use error_registry::BaseError;
use futures::future;
use log::{error, info};
use std::fmt::{Debug, format};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

#[async_trait]
pub trait Alivable: Sync + Send {
    async fn is_alive(&self) -> bool;
    async fn info(&self) -> &'static str;
}

#[derive(Clone)]
pub struct HealthcheckerServer {
    /// Determine status of program
    /// true - all okay
    /// false - something goes wrong, need restart
    health: Arc<AtomicBool>,
    services: Arc<Vec<Box<dyn Alivable>>>,
    /// Timeout between checks, in millis
    pub timeout: u64,
}

impl HealthcheckerServer {
    pub async fn new(
        host: &str,
        timeout: u64,
        services: Option<Vec<Box<dyn Alivable>>>,
    ) -> Result<Self, BaseError<()>> {
        let health_checker = Self {
            health: Arc::new(AtomicBool::new(true)),
            timeout,
            services: Arc::new(services.unwrap_or_default()),
        };

        let health_checker_replica = health_checker.clone();
        let host_str = String::from(host);
        tokio::spawn({
            async move { health_checker_replica.http_loop(host_str).await }
        });
        Ok(health_checker)
    }

    pub async fn http_loop(&self, host: String) {
        use tiny_http::{Server, Response};

        let server = Server::http(host).unwrap();

        loop {
            info!("Loop");
            for request in server.incoming_requests() {
                let is_ok = self.is_ok().await;
                let response = if is_ok {
                    Response::from_string("DEBUG_OK").with_status_code(200u16)
                } else {
                    Response::from_string("DEBUG_ERROR").with_status_code(500u16)
                };
                info!("Healthchecker request responding: {:?}", request.respond(response));
            }
            tokio::time::sleep(Duration::from_millis(500));
        }
    }

    pub fn get_health_cheker(&self) -> HealthChecker {
        HealthChecker {
            health: self.health.clone(),
        }
    }

    pub async fn is_alive(&self) {
        while self.health.load(Ordering::Acquire) {
            sleep(Duration::from_millis(self.timeout)).await;
        }
    }

    pub async fn is_ok(&self) -> bool {
        let mut is_alive = true;

        for service in self.services.iter() {
            is_alive = is_alive && service.is_alive().await;
        }
        self.health.load(Ordering::Acquire) && is_alive
    }

    pub fn make_sick(&self) {
        self.health.store(false, Ordering::SeqCst);
    }
}

#[derive(Clone)]
pub struct HealthChecker {
    pub(crate) health: Arc<AtomicBool>,
}

impl HealthChecker {
    pub fn make_sick<D: Debug>(&self, log: Option<D>) {
        error!("Made sick on: {:#?}", log);
        self.health.store(false, Ordering::SeqCst);
    }
}
