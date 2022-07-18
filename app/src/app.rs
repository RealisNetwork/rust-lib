
use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats};
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use log::LevelFilter;
use schemas::{Agent, Schema};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use transport::Transport;

/// app = push_service!(app, ServiceApp, SetMailingSubscriptionStatusService)?;
#[macro_export]
macro_rules! push_service {
    ($app:expr,$serviceApp:ident,$service:ident) => {
        $app.push_with_dependency::<$serviceApp<_, _, $service, _>, $service, _, _>().await
    };
}

#[async_trait]
pub trait Runnable: Send + Sync {
    async fn run(&mut self);
}

#[async_trait]
pub trait AsyncTryFrom<T>: Sized {
    type Error;
    async fn async_try_from(_: T) -> Result<Self, Self::Error>;
}

pub trait AbstractService<P: Agent, G: Schema>: Send + Sync {}

pub struct App<T: GetTransport<N> + GetHealthchecker, N: Transport + Sync + Send> {
    services: Vec<Box<Mutex<dyn Runnable>>>,
    dependency_container: Arc<T>,
    _marker: std::marker::PhantomData<N>,
}

pub trait GetTransport<N: Transport + Sync + Send> {
    fn get_transport(&self) -> Arc<N>;
}

pub trait GetHealthchecker {
    fn get_healthchecker(&self) -> HealthChecker;
}

impl<T, N> App<T, N>
where
    T: 'static + Clone + Send + Sync + GetTransport<N> + GetHealthchecker,
    N: 'static + Transport + Sync + Send,
{
    pub fn new(dependency_container: Arc<T>) -> Self {
        Self {
            services: vec![],
            dependency_container,
            _marker: Default::default(),
        }
    }

    pub fn push(mut self, service: impl Runnable + 'static) -> Self {
        self.services.push(Box::new(Mutex::new(service)));
        self
    }

    pub async fn push_with_dependency<AbstractApp, ServiceInner, P, G>(
        mut self,
    ) -> Result<Self, BaseError<Value>>
    where
        AbstractApp: 'static + Runnable + AsyncTryFrom<Arc<T>>,
        ServiceInner: 'static + From<Arc<T>> + AbstractService<P, G>,
        P: 'static + Agent,
        G: 'static + Schema,
    {
        self.services.push(Box::new(Mutex::new(
            AbstractApp::async_try_from(self.dependency_container.clone())
                .await
                .map_err(|_| {
                    BaseError::<Value>::from(CustomErrorType::Nats(Nats::FailedToSubscribe))
                })?,
        )));
        Ok(self)
    }

    pub fn init_logger_with_level(self, logger_level: LevelFilter) -> Self {
        env_logger::Builder::new().filter_level(logger_level).init();
        self
    }

    pub fn init_logger(self) -> Self {
        env_logger::Builder::from_env(env_logger::Env::new().filter_or("LOGGER_LEVEL", "debug"))
            .init();
        self
    }
}

#[async_trait]
impl<T: Clone + Send + Sync + GetTransport<N> + GetHealthchecker, N: Transport + Sync + Send>
    Runnable for App<T, N>
{
    async fn run(&mut self) {
        let services = self.services.drain(..);
        futures::future::join_all(services.into_iter().map(|service| {
            tokio::spawn(async move {
                service.lock().await.run().await;
            })
        }))
        .await;
    }
}
