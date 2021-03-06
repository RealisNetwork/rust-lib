use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, TypeTuple};

pub fn impl_gettable_macros(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    let name = &ast.ident;

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];

    let topic_fn = if attr.elems.len() > 3 {
        let topic = &attr.elems[3];
        quote! {
            fn topic() -> String {
                #topic.to_string()
            }
        }
    } else {
        let name_string = name.to_string();
        quote! {
            fn topic() -> String {
                #name_string.to_string().to_case(Case::Snake)
            }
        }
    };

    let gen = quote! {
        use convert_case::{Case, Casing};

        impl Gettable for #name {
            type Error = #error;
            type MessageParams = #params;
            type MessageReturn = #returns;

            #topic_fn

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
