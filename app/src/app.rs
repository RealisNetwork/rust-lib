use std::ops::Deref;
use std::sync::Arc;
use crate::{Service, ServiceApp};
use async_trait::async_trait;
use healthchecker::HealthChecker;
use log::LevelFilter;
use schemas::{Agent, Schema};
use serde_json::Value;
use tokio::sync::Mutex;
use transport::Transport;


#[async_trait]
pub trait Runnable: Send + Sync {
    async fn run(&mut self);
}

pub struct App<T: GetTransport<N> + GetHealthchecker, N: Transport + Sync + Send + Clone> {
    services: Vec<Box<Mutex<dyn Runnable>>>,
    dependency_container: Arc<T>,
    _marker: std::marker::PhantomData<N>,
}

pub trait GetTransport<N: Transport + Sync + Send + Clone> {
   fn get_transport(&self) -> N;
}

pub trait GetHealthchecker{
    fn get_healthchecker(&self) -> HealthChecker;
}


// #[allow(clippy::derivable_impls)]
// impl<T> Default for App<T> {
//     fn default() -> Self {
//         Self { services: vec![] }
//     }
// }

impl<T: Clone + Send + Sync + GetTransport<N> + GetHealthchecker, N: 'static + Transport + Sync + Send + Clone> App<T, N> {
    pub fn new(dependency_container: T) -> Self {
        Self
        {
            services: vec![],
            dependency_container,
            _marker: Default::default(),
        }
    }

    pub fn push(mut self, service: impl Runnable + 'static) -> Self {
        self.services.push(Box::new(Mutex::new(service)));
        self
    }

    // TODO: Remove unwrap
    pub async fn push_with_dependency<
        P: 'static +  Agent, G: 'static + Schema,
        ServiceInner: 'static + Runnable + From<T> + Service<P, G>,
    >
    (
        mut self,
    ) -> Self {
        self.services.push(Box::new(Mutex::new(
            ServiceApp::new(
                ServiceInner::from(self.dependency_container.clone()),
                Arc::new(self.dependency_container.get_transport().as_ref()),
                self.dependency_container.get_healthchecker(),
            )
            .await
            .unwrap(),
        )));
        self
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
impl<T: Clone + Send + Sync + GetTransport<N> + GetHealthchecker, N: Transport + Sync + Send + Clone> Runnable for App<T, N> {
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
