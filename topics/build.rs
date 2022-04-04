use config::env::{EnvLoaded, EnvLoadedError};
#[cfg(feature = "rebuild")]
use parser::loader::{git::GitLoader, loader::Loader};
use quote::{ToTokens, __private::TokenStream};
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "rebuild_local")]
    load_env();

    #[cfg(feature = "rebuild")]
    generate();
}

#[cfg(feature = "rebuild")]
fn generate() {
    let filename = "topics.rs";
    let path = "./src";

    let result: Result<GitLoader, EnvLoadedError> = EnvLoaded::load(None);

    match result.map(|loader| loader.load()) {
        Ok(Ok(topics)) => {
            let mut code = TokenStream::default();

            topics.iter().for_each(|topic| code.extend(topic.to_token_stream()));

            let out = &mut PathBuf::from(path);
            out.push(filename);
            std::fs::write(out, code.to_string()).unwrap();
        }
        Ok(Err(error)) => println!("Fail to load git repo: `{}`", error),
        Err(error) => println!("Fail to load env options: `{:?}`", error),
    }
}

#[cfg(feature = "rebuild_local")]
fn load_env() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let env_path = out_dir.ancestors().skip(5).next().unwrap().join(".env");
    dotenv::from_filename(env_path).unwrap();
}
