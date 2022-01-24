use parser::loader::{git::GitLoader, loader::Loader};
use quote::{ToTokens, __private::TokenStream};
use std::path::PathBuf;

fn main() {
    let filename = "topics.rs";
    let path = "./src";

    let mut code = TokenStream::default();

    let loader = GitLoader::new(
        String::from("RealisNetwork"),
        String::from("libs"),
        String::from("./agents/src"),
        String::from("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"),
        String::from("main"),
        String::from("topics.ts"),
    );

    loader
        .load()
        .unwrap()
        .iter()
        .for_each(|topic| code.extend(topic.to_token_stream()));

    let loader = GitLoader::new(
        String::from("RealisNetwork"),
        String::from("libs"),
        String::from("./topics/src"),
        String::from("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"),
        String::from("main"),
        String::from("index.ts"),
    );

    loader
        .load()
        .unwrap()
        .iter()
        .for_each(|topic| code.extend(topic.to_token_stream()));

    let out = &mut PathBuf::from(path);
    out.push(filename);
    std::fs::write(out, code.to_string()).unwrap();
}
