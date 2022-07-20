use crate::{Service, ServiceApp};
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use schemas::{Agent, Schema};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use transport::Transport;

#[async_trait]
pub trait Runnable: Send + Sync {
    async fn run(&mut self);
}

pub trait DependencyContainerParameter<T: Transport>: Clone + Sync + Send {
    fn get_transport(&self) -> Arc<T>;

    fn get_health_checker(&self) -> HealthChecker;
}

pub struct App<Dependency: DependencyContainerParameter<T>, T: Transport> {
    services: Vec<Box<Mutex<dyn Runnable>>>,
    dependency_container: Dependency,
    _marker: std::marker::PhantomData<T>,
}

impl<Dependency, T> App<Dependency, T>
where
    Dependency: DependencyContainerParameter<T> + 'static,
    T: Transport + 'static,
{
    pub async fn new(dependency_container: Dependency) -> Self {
        Self {
            services: vec![],
            dependency_container,
            _marker: Default::default(),
        }
    }

    /// Init `env_logger` from env_key variable
    /// if env_key not specified use `LOGGER_LEVEL`
    /// if such variable not found try to load it from .env
    pub fn init_logger(&mut self, env_key: Option<&str>) -> &mut Self {
        const LOGGER_ENV: &str = "LOGGER_LEVEL";
        // Setting env variable from .env file
        // because of env_logger cannot use .env
        if std::env::var(env_key.unwrap_or(LOGGER_ENV)).is_err() {
            if let Ok(value) = dotenv::var(env_key.unwrap_or(LOGGER_ENV)) {
                std::env::set_var(env_key.unwrap_or(LOGGER_ENV), value);
            }
        }
        // Init logger
        env_logger::init_from_env(
            env_logger::Env::new().filter_or(env_key.unwrap_or(LOGGER_ENV), "info"),
        );
        self
    }

    pub fn push(&mut self, service: impl Runnable + 'static) -> &mut Self {
        self.services.push(Box::new(Mutex::new(service)));
        self
    }

    // TODO: impl `push_runnable` for `Runnable` same as `push_service`

    /// Construct `Service` from `DependencyContainer` and wrap it into `ServiceApp`
    /// Require to specify:
    /// S - service handler struct
    /// Params - service input params
    /// Returns - service output
    pub async fn push_service<S, Params, Returns>(&mut self) -> Result<&mut Self, BaseError<Value>>
    where
        S: 'static + Service<Params, Returns> + From<Dependency>,
        Params: 'static + Agent,
        Returns: 'static + Schema,
    {
        let service: S = self.dependency_container.clone().into();
        let service_app = ServiceApp::new(
            service,
            self.dependency_container.get_transport(),
            self.dependency_container.get_health_checker(),
        )
        .await?;
        Ok(self.push(service_app))
    }
}

#[async_trait]
impl<Dependency: DependencyContainerParameter<T>, T: Transport> Runnable for App<Dependency, T> {
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
