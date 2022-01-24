use quote::quote;
use proc_macro::TokenStream;
use syn::{self, DeriveInput};

pub fn impl_to_json_macros(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident;
    quote! { impl ToJson for #name {} }.into()
}
