use config::env::env::EnvLoaded;
use schemas_generator::{
    agents::Agent,
    env_loader::{GitLoader, Loader},
};

fn main() {
    let git_loader = GitLoader::load(None).expect("Fail to load env");
    println!("{:?}", git_loader);

    let json: Vec<Agent> = git_loader.start().expect("Fail to load from Git");

    println!("{:#?}", json);
}
