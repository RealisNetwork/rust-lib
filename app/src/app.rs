pub use log::LevelFilter;

use async_trait::async_trait;
use tokio::sync::Mutex;

#[async_trait]
pub trait Runnable: Send + Sync {
    async fn run(&mut self);
}

#[derive(Default)]
pub struct App {
    services: Vec<Box<Mutex<dyn Runnable>>>,
}

impl App {
    pub fn push(mut self, service: impl Runnable + 'static) -> Self {
        self.services.push(Box::new(Mutex::new(service)));
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
impl Runnable for App {
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
