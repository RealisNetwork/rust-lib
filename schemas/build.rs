use config::env::env::EnvLoaded;
use schemas_generator::{
    agent::Agent,
    env_loader::{GitLoader, Loader},
};

fn main() {
    let git_loader = GitLoader::load(None).expect("Fail to load env");

    let _json: Vec<Agent> = git_loader.start().expect("Fail to load from Git");
}
