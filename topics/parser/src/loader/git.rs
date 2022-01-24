use crate::loader::loader::Loader;
use crate::topic::Topic;
use crate::ParseResult;
use crate::parse_line;
use tokio::runtime::Runtime;

pub struct GitLoader {
    owner: String,
    repo: String,
    root_path: String,
    token: String,
    branch: String,
    source_file: String,
}

impl GitLoader {
    #[must_use]
    pub fn new(owner: String, repo: String, root_path: String, token: String, branch: String, source_file: String) -> Self {
        Self {
            owner,
            repo,
            root_path,
            token,
            branch,
            source_file,
        }
    }

    fn get_file(&self, path: &str, file: &str) -> Result<String, ()> {
        let url = Runtime::new()
            .unwrap()
            .block_on(
                octocrab::OctocrabBuilder::new()
                .personal_token(self.token.clone())
                .build()
                .map_err(|_| ())? // TODO fix errors
                .repos(self.owner.clone(), self.repo.clone())
                .get_content()
                .path(path)
                .r#ref(self.branch.clone())
                .send()
            )
            .map_err(|_| ())? // TODO fix errors
            .items
            .into_iter()
            .find(|content| &content.name == file)
            .ok_or(())? // TODO fix errors
            .download_url
            .ok_or(())?; // TODO fix errors

        reqwest::blocking::get(url)
            .map_err(|_| ())? // TODO fix errors
            .text()
            .map_err(|_| ()) // TODO fix errors
    }

    fn read_file(&self, path: &str, file: &str) -> Result<Vec<ParseResult>, ()> {
        let file = self.get_file(path, file)?;

        let results = file.split('\n')
            .into_iter()
            .flat_map(|line| parse_line(line))
            .collect();

        Ok(results)
    }

    fn get_absolute_path(&self, path: &str) -> String {
        format!("{}{}", self.root_path, path.trim_start_matches('.'))
    }

    fn process_file(&self, path: &str, file: &str) -> Result<Vec<Topic>, ()> {
        let topics = self.read_file(path, file)?
            .into_iter()
            .map(|parse_result| {
                match parse_result {
                    ParseResult::Import(import) => self.process_file(&self.get_absolute_path(&import.get_path()), &import.get_file()).unwrap_or(vec![]), // TODO handle this unwrap()
                    ParseResult::Topic(topic) => vec![topic]
                }
            })
            .flatten()
            .collect();

        Ok(topics)
    }
}

impl Loader for GitLoader {
    fn load(self) -> Result<Vec<Topic>, ()> {
        self.process_file(&self.root_path, &self.source_file)
    }
}

#[cfg(test)]
mod tests {
    use crate::loader::git::GitLoader;
    use crate::loader::loader::Loader;

    impl Default for GitLoader {
        fn default() -> Self {
            Self {
                owner: String::from("RealisNetwork"),
                repo: String::from("libs"),
                root_path: String::from("./topics/src"),
                token: String::from("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"),
                branch: String::from("main"),
                source_file: String::from("index.ts"),
            }
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
        let git_loader = GitLoader::new(
            String::from("RealisNetwork"),
            String::from("libs"),
            String::from("./agents/src"),
            String::from("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"),
            String::from("main"),
            String::from("topics.ts"),
        );

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