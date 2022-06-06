use crate::parser::schema::Schema;
use config::env::EnvLoaded;
use serde_json::{json, Value};
use std::{fs::File, io::BufReader};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum SchemaLoaderError {
    Octocrab(octocrab::Error),
    FileNotFound(String),
    MissingDownloadUrl,
    Download(reqwest::Error),
    Parse(reqwest::Error),
    SerdeParse(serde_json::Error),
}

#[derive(Debug)]
pub struct SchemaLoader<T> {
    loader: T,
}

impl<T> SchemaLoader<T> {
    pub fn new(loader: T) -> Self {
        Self { loader }
    }
}

impl SchemaLoader<LoaderGit> {
    /// # Errors
    /// # Panics
    pub fn load(self) -> Result<Vec<Schema>, SchemaLoaderError> {
        let download_url = Runtime::new()
            .unwrap()
            .block_on(
                octocrab::OctocrabBuilder::new()
                    .personal_token(self.loader.token)
                    .build()
                    .map_err(SchemaLoaderError::Octocrab)?
                    .repos(self.loader.owner, self.loader.repo)
                    .get_content()
                    .path(self.loader.path)
                    .r#ref(self.loader.branch)
                    .send(),
            )
            .map_err(SchemaLoaderError::Octocrab)?
            .items
            .into_iter()
            .find(|content| content.name == self.loader.file_name)
            .ok_or(SchemaLoaderError::FileNotFound(self.loader.file_name))?
            .download_url
            .ok_or(SchemaLoaderError::MissingDownloadUrl)?;

        reqwest::blocking::get(download_url)
            .map_err(SchemaLoaderError::Download)?
            .json()
            .map_err(SchemaLoaderError::Parse)
    }
}

impl SchemaLoader<LoaderJson> {
    /// # Errors
    pub fn load(self) -> Result<Vec<Schema>, SchemaLoaderError> {
        serde_json::from_value(self.loader.json).map_err(SchemaLoaderError::SerdeParse)
    }
}

#[derive(Debug)]
pub struct LoaderGit {
    owner: String,
    repo: String,
    path: String,
    token: String,
    branch: String,
    file_name: String,
}

impl LoaderGit {
    #[must_use]
    pub fn new(owner: String, repo: String, path: String, token: String, branch: String, file_name: String) -> Self {
        Self {
            owner,
            repo,
            path,
            token,
            branch,
            file_name,
        }
    }
}

impl Default for LoaderGit {
    fn default() -> Self {
        Self {
            owner: String::from("RealisNetwork"),
            repo: String::from("libs"),
            path: String::from("./agents/lib"),
            token: String::load(Some(String::from("TOKEN"))).unwrap(),
            branch: String::from("main"),
            file_name: String::from("fullSchemas.json"),
        }
    }
}

pub struct LoaderJson {
    json: Value,
}

impl LoaderJson {
    /// # Panics
    #[must_use]
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let json = serde_json::from_reader(reader).unwrap();
        Self { json }
    }
}

impl Default for LoaderJson {
    fn default() -> Self {
        Self {
            json: json!([{
            "agent": "auth",
                "method": "auth_sendRequestToResetPassword",
                "byteMethod": "2",
                "byteAgent": "2",
                "params": {
                  "type": "string"
                },
                "returns": {
                  "type": "string"
                }
              }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schemas::schemas_loader::{LoaderGit, LoaderJson, SchemaLoader};

    #[test]
    fn load_with_default_settings() {
        let loader = LoaderGit::default();
        let schemas_result = SchemaLoader::new(loader).load();

        assert!(schemas_result.is_ok());
        assert!(schemas_result.unwrap().len() > 0);

        let loader = LoaderJson::default();
        let schemas_result = SchemaLoader::new(loader).load();

        assert!(schemas_result.is_ok());
        assert!(schemas_result.unwrap().len() > 0);
    }
}
