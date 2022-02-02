use crate::env::symbol::parse_lit_into_expr_path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Lit;

pub enum EnvDefaultAttrs {
    Default,
    DefaultPath(Lit),
    Empty,
}

impl EnvDefaultAttrs {
    pub fn postfix(&self) -> TokenStream {
        match self {
            EnvDefaultAttrs::Default => quote! { .unwrap_or_default() },
            EnvDefaultAttrs::DefaultPath(lit) => {
                let path = parse_lit_into_expr_path(&lit);
                quote! {.unwrap_or(#path()) }
            }
            EnvDefaultAttrs::Empty => quote! { ? },
        }
    }
}
