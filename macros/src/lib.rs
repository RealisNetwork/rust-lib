use proc_macro::TokenStream;
use quote::quote;
use serde_json::Value;
use syn::{self, DeriveInput, TypeTuple};

/// # Panics
#[proc_macro_derive(Gettable, attributes(gettable))]
pub fn gettable_macro_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    impl_gettable_macro(&ast, &attr)
}

fn impl_gettable_macro(ast: &DeriveInput, attr: &TypeTuple) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];

    let gen = quote! {
        use convert_case::{Case, Casing};

        impl Gettable for #name {
            type Error = #error;
            type MessageParams = #params;
            type MessageReturn = #returns;

            fn topic() -> String {
                #name_string.to_case(Case::Snake)
            }

            fn parse(payload: &[u8]) -> Result<Box<
            dyn Message<Params=Self::MessageParams, Return=Self::MessageReturn>
            >, Self::Error> {
                let message = serde_json::from_slice::<#name>(payload)?;
                Ok(Box::new(message))
            }
        }
    };
    gen.into()
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    impl_gettable_macro_errors(&ast, &attr)
}

fn impl_gettable_macro_errors(ast: &DeriveInput, attr: &TypeTuple) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];

    let gen = quote! {

        impl $name {
            fn enum_to_string(&self) -> String {
                match self {
                   $($name::$variant => format!("{}.{}", stringify!($name), stringify!($variant))),*
                }
            }

            fn error_to_value_with_message(&self, msg: &str) -> Value {
                match self {
                    $($name::$variant => {
                        json!({
                            "type": "Left",
                            "value": {
                                "msg": msg,
                                "type": format!("{}.{}", stringify!($name), stringify!($variant))
                            }
                        })
                    }),*
                }
            }

            fn error_to_value(&self) -> Value {
                match self {
                    $($name::$variant => {
                        json!({
                            "type": "Left",
                            "value": {
                                "type": format!("{}.{}", stringify!($name), stringify!($variant))
                            }
                        })
                    }),*
                }
            }
        }
    };
}
