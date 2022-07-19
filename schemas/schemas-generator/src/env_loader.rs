pub use anyhow::Result;
use config::env::{Env, EnvLoaded};
use error_registry::BaseError;
use octocrab::OctocrabBuilder;
use reqwest::blocking;
use serde::Deserialize;
use tokio::runtime::Runtime;

pub trait Loader {
    fn start<T: for<'a> Deserialize<'a>>(self) -> Result<T, String>;
}

#[derive(Env, Debug)]
pub struct GitLoader {
    pub root_agents_path: String,
    pub source_agents_file: String,
    pub token: String,
    pub repo: String,
    pub owner: String,
    pub branch: String,
}

impl GitLoader {
    #[must_use]
    pub fn new(
        root_agents_path: String,
        source_agents_file: String,
        token: String,
        repo: String,
        owner: String,
        branch: String,
    ) -> Self {
        Self {
            root_agents_path,
            source_agents_file,
            token,
            repo,
            owner,
            branch,
        }
    }
}

impl Default for GitLoader {
    fn default() -> Self {
        Self {
            owner: String::from("RealisNetwork"),
            repo: String::from("libs"),
            root_agents_path: String::from("./agents/lib"),
            token: String::load(Some(String::from("TOKEN"))).unwrap(),
            branch: String::from("main"),
            source_agents_file: String::from("allSchemas.json"),
        }
    }
}

// #[async_trait]
impl Loader for GitLoader {
    fn start<T: for<'a> Deserialize<'a>>(self) -> Result<T, String> {
        let url = Runtime::new()
            .map_err(|error| format!("Cannot create runtime: {:?}", error))?
            .block_on(
                OctocrabBuilder::new()
                    .personal_token(self.token.to_string())
                    .build()
                    .map_err(|error| format!("Cannot  error: {:?}", error))?
                    .repos(self.owner, self.repo)
                    .get_content()
                    .path(&self.root_agents_path)
                    .r#ref(self.branch)
                    .send(),
            )
            .map_err(|error| format!("{:?}", error))?
            .items
            .into_iter()
            .find(|content| content.name == self.source_agents_file)
            .ok_or(format!(
                "Missing file: {}, by path: {}!",
                self.source_agents_file, self.root_agents_path
            ))?
            .download_url
            .ok_or_else(|| String::from("Missing download url!"))?;

        blocking::get(url)
            .map_err(|error| format!("{:?}", error))?
            .json()
            .map_err(|error| format!("{:?}", error))
    }
}
