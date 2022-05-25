use std::collections::HashMap;
use std::fmt::Debug;
use convert_case::Casing;
#[cfg(feature = "rebuild")]
use serde::{Serialize, Deserialize};



fn main() {
    #[cfg(feature = "rebuild")]
    rebuild();
}

#[cfg(feature = "rebuild")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Error {
    header: String,
    params: ErrorParams,
}

#[cfg(feature = "rebuild")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorParams {
    err_type: String,
    code: u64,
}

#[cfg(feature = "rebuild")]
pub fn get_file(path: &str, file: &str,token: &str, repo: &str, owner: &str, branch: &str) -> Result<String, String> {
    use reqwest::blocking;
    use tokio::runtime::Runtime;
    use octocrab::OctocrabBuilder;


    let url = Runtime::new()
        .map_err(|error| format!("{:?}", error))?
        .block_on(
            OctocrabBuilder::new()
                .personal_token(token.to_string().clone())
                .build()
                .map_err(|error| format!("{:?}", error))?
                .repos(owner.clone(), repo.clone())
                .get_content()
                .path(path)
                .r#ref(branch.clone())
                .send(),
        )
        .map_err(|error| format!("{:?}", error))?
        .items
        .into_iter()
        .find(|content| &content.name == file)
        .ok_or(format!("Missing file: {}, by path: {}!", file, path))?
        .download_url
        .ok_or(String::from("Missing download url!"))?;

    blocking::get(url)
        .map_err(|error| format!("{:?}", error))?
        .text()
        .map_err(|error| format!("{:?}", error))
}


#[cfg(feature = "rebuild")]
pub fn rebuild () {
    use serde_json::Value;


    let json = get_file(
        dotenv::var("GIT_LOADER_ROOT_ERROR_REGISTRY").unwrap().parse::<String>().unwrap().as_str(),
        dotenv::var("GIT_LOADER_ROOT_ERROR_REGISTRY_FILE").unwrap().parse::<String>().unwrap().as_str(),
        dotenv::var("GIT_LOADER_TOKEN").unwrap().parse::<String>().unwrap().as_str(),
        dotenv::var("GIT_LOADER_REPO").unwrap().parse::<String>().unwrap().as_str(),
        dotenv::var("GIT_LOADER_OWNER").unwrap().parse::<String>().unwrap().as_str(),
        dotenv::var("GIT_LOADER_BRANCH").unwrap().parse::<String>().unwrap().as_str());

    if let Err(e) = json {
        panic!("{:#?}", e);
    }
    let json = json.unwrap();

    let map : HashMap<String, HashMap<String, Value>> = serde_json::from_str(json.as_str()).unwrap();

    let mut res2 : HashMap<String, Vec<ErrorParams>> = HashMap::new();

    for (header, params) in map {

        let mut head: Vec<String> = header.split(".").map(|s| s.to_string()).collect(); // index 0 - prefix, index 1 - postfix

        if res2.contains_key(&head[0]) {

            res2.get_mut(head[0].as_str()).unwrap().push(ErrorParams { err_type: head[1].clone(), code: params["code"].as_u64().unwrap(),});

        } else {
            res2.insert(head[0].clone(), vec![ ErrorParams { err_type: head[1].clone(), code: params["code"].as_u64().unwrap(), }]);
        }
    }


    panic!("{:#?}", res2);

}


