#![allow(unused_imports)]
use convert_case::{Case, Casing};
use quote::{
    __private::{Ident, Span, TokenStream},
    quote, ToTokens, TokenStreamExt,
};
#[cfg(feature = "rebuild")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use syn::parse::Parser;
use tokio::io::AsyncBufReadExt;

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
    code: u32,
    full_name: String,
}

#[cfg(feature = "rebuild")]
pub fn get_file(
    path: &str,
    file: &str,
    token: &str,
    repo: &str,
    owner: &str,
    branch: &str,
) -> Result<String, String> {
    use octocrab::OctocrabBuilder;
    use reqwest::blocking;
    use tokio::runtime::Runtime;

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
pub fn rebuild() {
    use serde_json::Value;

    let json = get_file(
        dotenv::var("GIT_LOADER_ROOT_ERROR_REGISTRY")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
        dotenv::var("GIT_LOADER_ROOT_ERROR_REGISTRY_FILE")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
        dotenv::var("GIT_LOADER_TOKEN")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
        dotenv::var("GIT_LOADER_REPO")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
        dotenv::var("GIT_LOADER_OWNER")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
        dotenv::var("GIT_LOADER_BRANCH")
            .unwrap()
            .parse::<String>()
            .unwrap()
            .as_str(),
    );

    if let Err(e) = json {
        panic!("{:#?}", e);
    }
    let json = json.unwrap();

    let map: HashMap<String, HashMap<String, Value>> = serde_json::from_str(json.as_str()).unwrap();

    let mut res2: HashMap<String, Vec<ErrorParams>> = HashMap::new();

    for (header, params) in map {
        let mut head: Vec<String> = header.split(".").map(|s| s.to_string()).collect(); // index 0 - prefix, index 1 - postfix

        if res2.contains_key(&head[0]) {
            res2.get_mut(head[0].as_str()).unwrap().push(ErrorParams {
                err_type: head[1].clone(),
                code: params["code"].as_u64().unwrap() as u32,
                full_name: header.clone(),
            });
        } else {
            res2.insert(
                head[0].clone(),
                vec![ErrorParams {
                    err_type: head[1].clone(),
                    code: params["code"].as_u64().unwrap() as u32,
                    full_name: header.clone(),
                }],
            );
        }
    }

    let mut code = TokenStream::default();

    let general_enum_name = Ident::new(
        "GeneratedError".to_case(Case::UpperCamel).as_str(),
        Span::call_site(),
    );
    let mut general_enum = TokenStream::default();

    let field_name = res2
        .keys()
        .into_iter()
        .map(|field| Ident::new(field.to_case(Case::UpperCamel).as_str(), Span::call_site()));

    let field_name2 = res2
        .keys()
        .into_iter()
        .map(|field| Ident::new(field.to_case(Case::UpperCamel).as_str(), Span::call_site()));

    let field_name3 = field_name.clone();
    let field_name_subfield = res2
        .keys()
        .into_iter()
        .map(|field| Ident::new(field.to_case(Case::Snake).as_str(), Span::call_site()));

    general_enum.extend(quote!(
        use serde::{Serialize, Deserialize};
        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
        #[serde(untagged)]
        pub enum #general_enum_name{
            #(#field_name(#field_name),)*
        }
    ));

    for enum_name in res2.keys() {
        let enum_name_indent = Ident::new(
            enum_name.to_case(Case::UpperCamel).as_str(),
            Span::call_site(),
        );
        let postfix = res2.get(enum_name).unwrap().iter().map(|field| {
            return match field.err_type.parse::<u32>() {
                Ok(_) => Ident::new(
                    format!("E{}", field.err_type.to_case(Case::UpperCamel)).as_str(),
                    Span::call_site(),
                ),
                Err(e) => Ident::new(
                    field.err_type.to_case(Case::UpperCamel).as_str(),
                    Span::call_site(),
                ),
            };
        });

        let postifx2 = postfix.clone();
        let postifx3 = postfix.clone();
        let postifx4 = postfix.clone();
        let full_names = res2
            .get(enum_name)
            .unwrap()
            .iter()
            .map(|field| field.full_name.clone());
        let error_code = res2
            .get(enum_name)
            .unwrap()
            .iter()
            .map(|field| field.code.clone());
        let full_names2 = full_names.clone();

        code.extend(quote!(
            #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
            pub enum #enum_name_indent {
                #(
                    #[serde(rename = #full_names2)]
                    #postfix,
                )*
            }
            impl ToString for #enum_name_indent {
                fn to_string(&self) -> String {
                    match self {
                        #(
                            Self::#postifx2=>#full_names.to_string(),
                        )*
                    }
                }
            }

            impl From<#enum_name_indent> for u32 {
                fn from(error_type: #enum_name_indent) -> u32 {
                    match error_type {
                        #(
                            #enum_name_indent::#postifx4=>#error_code,
                        )*
                    }
                }

            }

        ));
    }

    general_enum.extend(quote!(
        #(
            impl From<#field_name2> for #general_enum_name {
                fn from(error: #field_name2) -> Self {
                    #general_enum_name::#field_name2(error)
                }
            }
        )*
        impl From<#general_enum_name> for u32 {
            fn from(error_type: #general_enum_name) -> u32 {
                match error_type {
                    #(#general_enum_name::#field_name3(#field_name_subfield) => { u32::from(#field_name_subfield) })*
            }
        }
    }
    ));

    let header = "/// This errors are autogenerated. In case you want to add any \
    custom error, please use Custom file, otherwise all changes will be deleted on next \
    recompilation \n";

    std::fs::write(
        "./src/generated_errors.rs",
        format!(
            "{}\n\n{}\n{}",
            header,
            general_enum.to_string(),
            code.to_string()
        ),
    );
}
