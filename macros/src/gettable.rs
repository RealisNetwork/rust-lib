use proc_macro::TokenStream;
use convert_case::{Case, Casing};
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{self, DeriveInput, TypeTuple};

pub fn impl_gettable_macros(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    let name = &ast.ident;
    let mut name_string = name.to_string().to_case(Case::Snake);

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];
    if attr.elems.len() > 3 {
        name_string = syn::parse2::<Literal>(attr.elems[3].to_token_stream()).unwrap().to_string();
    }

    let gen = quote! {
        use convert_case::{Case, Casing};

        impl Gettable for #name {
            type Error = #error;
            type MessageParams = #params;
            type MessageReturn = #returns;

            fn topic() -> String {
                #name_string.to_string
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
