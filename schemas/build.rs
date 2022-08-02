#[cfg(feature = "rebuild")]
fn main() {
    use config::env::EnvLoaded;
    use quote::__private::Ident;
    use quote::{quote, ToTokens};
    use schemas_generator::{
        agent::Agent,
        env_loader::{GitLoader, Loader},
    };
    use std::collections::HashSet;
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };
    use syn::__private::Span;

    const PATH: &str = "src/generated_schemas/";
    const MOD_RS: &str = "mod.rs";

    let git_loader = GitLoader::load(None).expect("Fail to load env");

    let agents: Vec<Agent> = git_loader.start().expect("Fail to load from Git");

    // TODO: Check this
    let header =
        "// This file are autogenerated on build, everything you write here will be replaced";

    for agent in agents
        .clone()
        .into_iter()
        .map(|agent| agent.create_directory_name())
        .collect::<HashSet<_>>()
    {
        // Creating directories
        std::fs::create_dir_all(format!("{}{}", PATH, agent)).expect("Fail to create directory");
        // Creating mod.rs in each directory
        let mut mod_file =
            std::fs::File::create(&Path::new(&format!("{}{}/{}", PATH, agent, MOD_RS)))
                .expect("Fail to create \"mod.rs\" file");
        // Creating `pub mod ...`
        let agent_methods = agents
            .iter()
            .filter(|a| a.create_directory_name() == agent)
            .map(|agent| Ident::new(&agent.create_file_name(), Span::call_site()))
            .collect::<Vec<_>>();
        let pub_mods = quote! {
          #( pub mod #agent_methods; )*
        };
        mod_file
            .write_all(pub_mods.to_string().as_bytes())
            .expect("Fail to write to \"mod.rs\"");

        // Creating `pub use ...::*
        // Maybe get rid of this?
        let agent_methods = agents
            .iter()
            .filter(|a| a.agent == agent)
            .map(|agent| Ident::new(&agent.create_file_name(), Span::call_site()))
            .collect::<Vec<_>>();
        let pub_mods = quote! {
          #( pub use #agent_methods::*; )*
        };
        mod_file
            .write_all(pub_mods.to_string().as_bytes())
            .expect("Fail to write to \"mod.rs\"");
    }

    // Creating src/generated_schemas/mod.rs in
    let mut mod_file = std::fs::File::create(&Path::new(&format!("{}{}", PATH, MOD_RS)))
        .expect("Fail to create \"mod.rs\" file");
    let values = agents
        .clone()
        .into_iter()
        .map(|agent| agent.create_directory_name())
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|agent| Ident::new(&agent, Span::call_site()));

    let pub_mod = quote! {
      #( pub mod #values; )*
      pub mod prelude;
    };
    mod_file
        .write_all(pub_mod.to_string().as_bytes())
        .expect("Fail to write to \"mod.rs\"");

    // Creating src/generated_schemas/prelude.rs
    let mut prelude_file = std::fs::File::create(&Path::new(&format!("{}prelude.rs", PATH)))
        .expect("Fail to create \"prelude.rs\" file");
    let pub_prelude = quote! {
      pub use serde::{Serialize, Deserialize};
      pub use serde_json::Value;
      pub use crate::{Agent, Schema};
      pub use serde::de::Deserializer;
    };
    prelude_file
        .write_all(pub_prelude.to_string().as_bytes())
        .expect("Fail to write to \"prelude.rs\"");

    for schema in agents {
        let agent = schema.create_directory_name();
        let method = schema.create_file_name();

        let code = schema.into_token_stream();

        let out = &mut PathBuf::from(format!("{}{}", PATH, agent));
        out.push(format!("{}.rs", method));
        let content = format!("{}\n\n{}", header, code);

        std::fs::write(out, content).unwrap();
    }
}

#[cfg(not(feature = "rebuild"))]
fn main() {}
