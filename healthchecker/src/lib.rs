use std::io;

use error_registry::BaseError;
use futures::future;
use tokio::time::{sleep, Duration};
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct HealthChecker {
    /// Determine status of program
    /// true - all okay
    /// false - something goes wrong, need restart
    pub health: Arc<AtomicBool>,
    /// Timeout between checks, in millis
    pub timeout: u64,
}

impl HealthChecker {
    pub async fn new(host: &String, timeout: u64) -> Result<Self, BaseError<()>> {
        let health_checker = Self {
            health: Arc::new(AtomicBool::new(true)),
            timeout,
        };
        let addr = host.parse()?;
        tokio::spawn({
            let health_checker = health_checker.clone();
            async move { TcpServer::new(Http, addr).serve(move || Ok(health_checker.clone())) }
        });
        Ok(health_checker)
    }

    pub async fn is_alive(&self) {
        while self.health.load(Ordering::Acquire) {
            sleep(Duration::from_millis(self.timeout)).await;
        }
    }

    pub fn make_sick(&self) {
        self.health.store(false, Ordering::SeqCst);
    }

    pub fn is_ok(&self) -> bool {
        self.health.load(Ordering::Acquire)
    }
}

impl Service for HealthChecker {
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;
    type Request = Request;
    type Response = Response;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        if self.health.load(Ordering::Acquire) {
            resp.status_code(200, "OK");
            resp.body("DEBUG_OK");
        } else {
            resp.status_code(500, "Internal Server Error");
            resp.body("DEBUG_ERROR");
        }
        future::ok(resp)
    }
}
