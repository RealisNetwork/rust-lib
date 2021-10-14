use std::io;

use futures::future;
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub async fn listen(health: Arc<AtomicBool>, host: &String) {
    tokio::spawn({
        let host = host.clone();
        async move {
            TcpServer::new(Http, host.parse().unwrap())
                .serve(move || Ok(HealthChecker::new(Arc::clone(&health))));
        }
    });
}

pub struct HealthChecker {
    /// Determine status of program
    /// true - all okay
    /// false - something goes wrong, need restart
    health: Arc<AtomicBool>,
}

impl HealthChecker {
    pub fn new(health: Arc<AtomicBool>) -> Self {
        Self { health }
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
