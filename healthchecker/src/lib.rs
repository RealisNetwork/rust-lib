use async_trait::async_trait;
use error_registry::BaseError;
use log::{error, info};
use std::fmt::{Debug, format};
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::process::Output;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use hyper::{Body, Server, Response, Request, StatusCode, http};
use hyper::service::Service;
use tokio::time::{sleep, Duration};
use futures_util::future;

const ROOT: &str = "/";

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
            async move { health_checker_replica.http_init(host_str).await }
        });
        Ok(health_checker)
    }

    pub async fn http_init(self, host: String) {

        let addr = host.parse().unwrap();

        let server = Server::bind(&addr).serve(HealthcheckerHTTP {healthchecker: self});

        println!("Listening on http://{}", addr);

        server.await.unwrap();
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

pub struct Svc {
    healthchecker: HealthcheckerServer,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();

        let healthchecker = self.healthchecker.clone();

        let fut = async move{
            let state = healthchecker.is_ok().await;
            let rsp = if state {
                rsp.status(200).body(Body::from(Vec::from(format!("DEBUG_OK")))).unwrap()
            } else {
                rsp.status(500).body(Body::from(Vec::from(format!("DEBUG_ERROR")))).unwrap()
            };
            
            Ok(rsp)
        };

        Box::pin(fut)
    }
}

pub struct HealthcheckerHTTP {
    healthchecker: HealthcheckerServer,
}

impl<T> Service<T> for HealthcheckerHTTP {

    type Response = Svc;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(Svc {healthchecker: self.healthchecker.clone()})
    }
}