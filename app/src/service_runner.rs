use std::future::Future;
use serde_json::Value;
use error_registry::BaseError;

pub struct ServiceRunner {
    blocking_treads: usize,
    workers_number: usize,
}

impl ServiceRunner{
    pub fn build_with_params(blocking_treads: usize, workers_number: usize) -> Self {
        Self {
            blocking_treads,
            workers_number,
        }
    }

    pub fn run<Fut,T>(self, run: impl FnOnce(T) -> Fut, env_config: T)
        where Fut: Future<Output=Result<(), BaseError<Value>>> {
        
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.workers_number)
            .max_blocking_threads(self.blocking_treads)
            .enable_all()
            .build()
            .unwrap();

        let handle = rt.handle().clone();
        let (sx, rx) = tokio::sync::oneshot::channel();

        rt.block_on(async move {
            if let Err(error) = run(env_config).await
            {
                if error.is_critical() {
                    log::error!("Service critical error! Error msg: {}", error);
                    sx.send(0).expect("Sender failed...");
                } else {
                    log::error!("Error msg: {}", error);
                }
            }
        });

        handle.spawn(async move {
            // error listener

            let result = rx.await.unwrap();


            if result == 0 {
                log::info!("Service shutdown...");
                rt.shutdown_background();
            }
        });
    }
}


impl Default for ServiceRunner {
    fn default() -> Self {
        Self {
            blocking_treads: 512,
            workers_number: 50,
        }
    }
}