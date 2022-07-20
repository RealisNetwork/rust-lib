use async_trait::async_trait;
use error_registry::BaseError;
use futures_util::future;
use hyper::service::Service;
use hyper::{http, Body, Request, Response, Server};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;

/// Wrap services in this wrapper in order to conveniently add them to healthchecker
pub struct Wrapper<T: Alivable>(Arc<T>);

#[async_trait]
impl<T: Alivable> Alivable for Wrapper<T> {
    async fn is_alive(&self) -> bool {
        self.0.is_alive().await
    }

    async fn info(&self) -> &'static str {
        self.0.info().await
    }
}

impl<T: Alivable> From<T> for Wrapper<T> {
    fn from(other: T) -> Wrapper<T> {
        Wrapper(Arc::new(other))
    }
}

impl<T: Alivable> From<Arc<T>> for Wrapper<T> {
    fn from(other: Arc<T>) -> Wrapper<T> {
        Wrapper(other)
    }
}

/// Implement this trait on your service structure or any service that can loose connection or etc.
/// Object of this trait can be passed as services to healthchecker in oreder to check em all.
#[async_trait]
pub trait Alivable: Sync + Send {
    async fn is_alive(&self) -> bool;
    async fn info(&self) -> &'static str;
}

/// Base of healthchecker, should be made one time, in case you have to control state of
/// other services, lightweight instances of healthchecker can be obtained using get_health_cheker()
/// In case state of healthchecker is sick or any service in service vector is sick
/// http request to healthchecker will return 500 code, if everything is fine, it will return 200
#[derive(Clone)]
pub struct HealthcheckerServer {
    /// Determine status of program
    /// true - all okay
    /// false - something goes wrong, need restart
    health: Arc<AtomicBool>,
    host: String,
    services: Arc<Mutex<Vec<Box<dyn Alivable>>>>,
}

impl HealthcheckerServer {
    pub async fn new(
        services: Option<Vec<Box<dyn Alivable>>>,
        host: &str,
    ) -> Result<Self, BaseError<()>> {
        let health_checker = Self {
            health: Arc::new(AtomicBool::new(true)),
            host: host.to_string(),
            services: Arc::new(Mutex::new(services.unwrap_or_default())),
        };

        Ok(health_checker)
    }

    pub fn run(self) {
        let health_checker_replica = self.clone();
        tokio::spawn(async move {
            let addr = self.host.parse().unwrap();

            let server = Server::bind(&addr).serve(HealthcheckerHTTPBuilder {
                healthchecker: health_checker_replica,
            });

            log::info!("Run healthchecker on http://{}", addr);

            server.await.unwrap();
        });
    }

    /// Add any struct that implements Alivable to vector of checkable services (all the services
    /// will be checked when is_ok() method is executed)
    pub async fn push(&mut self, s: Box<dyn Alivable>) {
        self.services.lock().await.push(s);
    }

    /// Add any struct that implements Alivable to vector of checkable services (all the services
    /// will be checked when is_ok() method is executed)
    pub async fn add<T: 'static + Alivable>(self, s: Wrapper<T>) -> Self {
        self.services.lock().await.push(Box::new(s));
        self
    }

    /// Returns lightweight structure, witch can change inner state of healthchecker
    pub fn get_health_cheker(&self) -> HealthChecker {
        HealthChecker {
            health: self.health.clone(),
        }
    }

    /// Checks every single service in the list and checks inner state
    pub async fn is_ok(&self) -> bool {
        if self.health.load(Ordering::Acquire) {
            for service in self.services.lock().await.iter() {
                if !service.is_alive().await {
                    log::error!("Healthchecker fallen by reason: {}", service.info().await);
                    return false;
                }
            }
        } else {
            return false;
        }
        true
    }

    /// Toggle state of healthchecker to not alive
    pub fn make_sick(&self) {
        self.health.store(false, Ordering::SeqCst);
    }
}

/// Lightweight simple structure for setting up the state of healthchecker
#[derive(Clone)]
pub struct HealthChecker {
    pub health: Arc<AtomicBool>,
}

impl HealthChecker {
    /// Toggle state of healthchecker
    pub fn make_sick<D: Debug>(&self, log: Option<D>) {
        log::error!("Made sick on: {:#?}", log);
        self.health.store(false, Ordering::SeqCst);
    }
}

impl Service<Request<Body>> for HealthcheckerServer {
    type Response = Response<Body>;
    type Error = http::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    #[allow(unused_variables)]
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();

        let healthchecker = self.clone();

        // Prepare future to response
        let fut = async move {
            let state = healthchecker.is_ok().await;
            let rsp = if state {
                rsp.status(200)
                    .body(Body::from(Vec::from("DEBUG_OK")))
                    .unwrap()
            } else {
                rsp.status(500)
                    .body(Body::from(Vec::from("DEBUG_ERROR")))
                    .unwrap()
            };

            Ok(rsp)
        };

        Box::pin(fut)
    }
}

/// Dispatch HTTP request to service
pub struct HealthcheckerHTTPBuilder {
    healthchecker: HealthcheckerServer,
}

impl<T> Service<T> for HealthcheckerHTTPBuilder {
    type Response = HealthcheckerServer;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(self.healthchecker.clone())
    }
}
