use std::io;

use futures::future;
use tokio::time::{sleep, Duration};
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::task::JoinHandle;

#[derive(Clone, Copy)]
pub struct HealthChecker {
    /// Determine status of program
    /// true - all okay
    /// false - something goes wrong, need restart
    health: Arc<AtomicBool>,
}

impl HealthChecker {
    pub async fn new(host: &String) -> Self {
        let health_checker = Self{health: Arc::new(AtomicBool::new(true))};
        tokio::spawn({
            let host = host.clone();
            let health_checker = health_checker.clone();
            async move {
                TcpServer::new(Http, host.parse().unwrap())
                    .serve(move || Ok(health_checker.clone()))
            }
        });
        health_checker
    }

    pub async fn is_alive(&self) {
        while self.health.load(Ordering::Acquire) {
            sleep(Duration::from_millis(10000)).await;
        }
    }

    pub fn make_sick(&self) {
        self.health.store(false, Ordering::SeqCst);
    }
}

impl Service for HealthChecker {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

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
