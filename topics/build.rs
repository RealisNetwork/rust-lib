use parser::loader::{git::GitLoader, loader::Loader};
use quote::{ToTokens, __private::TokenStream};
use std::path::PathBuf;
use config::env::EnvLoaded;

fn main() {
    let filename = "topics.rs";
    let path = "./src";

    let mut code = TokenStream::default();

    let loader: GitLoader = EnvLoaded::load("").unwrap();

    loader
        .load()
        .unwrap()
        .iter()
        .for_each(|topic| code.extend(topic.to_token_stream()));

    let out = &mut PathBuf::from(path);
    out.push(filename);
    std::fs::write(out, code.to_string()).unwrap();
}
