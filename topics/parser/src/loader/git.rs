use crate::{
    loader::loader::Loader,
    topic::Topic,
    utils::{parse_line, ParseResult},
};
use config::env::{Env, EnvLoaded, EnvLoadedError};
use tokio::runtime::Runtime;

#[derive(Env)]
pub struct GitLoader {
    owner: String,
    repo: String,
    root_agents_path: String,
    root_topics_path: String,
    token: String,
    branch: String,
    source_agents_file: String,
    source_topics_file: String,
}

impl GitLoader {
    #[must_use]
    pub fn new(
        owner: String,
        repo: String,
        root_agents_path: String,
        root_topics_path: String,
        token: String,
        branch: String,
        source_agents_file: String,
        source_topics_file: String,
    ) -> Self {
        Self {
            owner,
            repo,
            root_agents_path,
            root_topics_path,
            token,
            branch,
            source_agents_file,
            source_topics_file,
        }
    }

    fn get_file(&self, path: &str, file: &str) -> Result<String, String> {
        let url = Runtime::new()
            .map_err(|error| format!("{:?}", error))?
            .block_on(
                octocrab::OctocrabBuilder::new()
                    .personal_token(self.token.clone())
                    .build()
                    .map_err(|error| format!("{:?}", error))?
                    .repos(self.owner.clone(), self.repo.clone())
                    .get_content()
                    .path(path)
                    .r#ref(self.branch.clone())
                    .send(),
            )
            .map_err(|error| format!("{:?}", error))?
            .items
            .into_iter()
            .find(|content| &content.name == file)
            .ok_or(format!("Missing file: {}, by path: {}!", file, path))?
            .download_url
            .ok_or(String::from("Missing download url!"))?;

        reqwest::blocking::get(url)
            .map_err(|error| format!("{:?}", error))?
            .text()
            .map_err(|error| format!("{:?}", error))
    }

    fn read_file(&self, path: &str, file: &str) -> Result<Vec<ParseResult>, String> {
        let file = self.get_file(path, file)?;

        let results = file.split('\n').into_iter().flat_map(|line| parse_line(line)).collect();

        Ok(results)
    }

    fn get_absolute_path(root: &str, path: &str) -> String {
        format!("{}{}", root, path.trim_start_matches('.'))
    }

    fn process_file(&self, root: &str, path: &str, file: &str) -> Result<Vec<Topic>, String> {
        let topics = self
            .read_file(path, file)?
            .into_iter()
            .map(|parse_result| {
                match parse_result {
                    ParseResult::Import(import) => self
                        .process_file(root, &Self::get_absolute_path(root, &import.get_path()), &import.get_file())
                        .unwrap_or(vec![]), // TODO handle this unwrap()
                    ParseResult::Topic(topic) => vec![topic],
                }
            })
            .flatten()
            .collect();

        Ok(topics)
    }
}

impl Loader for GitLoader {
    fn load(self) -> Result<Vec<Topic>, String> {
        let agents = self.process_file(&self.root_agents_path, &self.root_agents_path, &self.source_agents_file)?;
        let topics = self.process_file(&self.root_topics_path, &self.root_topics_path, &self.source_topics_file)?;
        Ok(agents.into_iter().chain(topics.into_iter()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::loader::{git::GitLoader, loader::Loader};
    use config::env::EnvLoaded;

    impl Default for GitLoader {
        fn default() -> Self {
            EnvLoaded::load(None).unwrap()
        }
    }

    #[test]
    fn topics_package() {
        let git_loader = GitLoader::default();

        let result = git_loader.load();

        match result {
            Ok(ref values) => {
                for v in values {
                    println!("{:?}", v);
                }
            }
            Err(_) => {}
        }

        assert!(result.is_ok())
    }

    #[test]
    fn agents_package() {
        let git_loader = EnvLoaded::load(None).unwrap();
    
        let result = git_loader.load();
    
        match result {
            Ok(ref values) => {
                for v in values {
                    println!("{:?}", v);
                }
            }
            Err(_) => {}
        }
    
        assert!(result.is_ok())
    }
}
