use async_trait::async_trait;
use log::LevelFilter;
use tokio::sync::Mutex;

// pub trait DependencyContainable{};

#[async_trait]
pub trait Runnable: Send + Sync {
    async fn run(&mut self);
}

pub struct App<T> {
    services: Vec<Box<Mutex<dyn Runnable>>>,
    dependency_container: T,
}

// #[allow(clippy::derivable_impls)]
// impl<T> Default for App<T> {
//     fn default() -> Self {
//         Self { services: vec![] }
//     }
// }



impl<T: Clone + Send + Sync> App <T>{
    pub fn new(dependency_container: T) -> Self {
        Self {
            services: vec![],
            dependency_container,
        }
    }

    pub fn push(mut self, service: impl Runnable + 'static) -> Self {
        self.services.push(Box::new(Mutex::new(service)));
        self
    }

    pub fn push_with_dependency<Service: 'static + Runnable + From<T>>(mut self) -> Self {
        self.services.push(Box::new(Mutex::new(Service::from(self.dependency_container.clone()))));
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
impl<T: Clone + Send + Sync> Runnable for App<T> {
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
