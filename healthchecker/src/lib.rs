use async_trait::async_trait;
use error_registry::BaseError;
use futures::future;
use log::error;
use std::fmt::Debug;
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

//
// impl Aliveable for Transport {
//     async fn is_alive(&self) -> bool {
//         todo!()
//     }
//
//     async fn info(&self) -> &'static str {
//         todo!()
//     }
// }

//
// impl Aliveable for Database {
// ...
// }

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

        let addr = host.parse()?;

        tokio::spawn({
            let health_checker = health_checker.clone();
            async move { TcpServer::new(Http, addr).serve(move || Ok(health_checker.clone())) }
        });
        Ok(health_checker)
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

impl Service for HealthcheckerServer {
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;
    type Request = Request;
    type Response = Response;

    /// WARNING! Use ONLY in context of tokio runtime, in other case it will cause panic
    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();

        let is_alive = tokio::runtime::Handle::current().block_on(self.is_ok());
        if is_alive {
            resp.status_code(200, "OK");
            resp.body("DEBUG_OK");
        } else {
            resp.status_code(500, "Internal Server Error");
            resp.body("DEBUG_ERROR");
        }
        future::ok(resp)
    }
}
