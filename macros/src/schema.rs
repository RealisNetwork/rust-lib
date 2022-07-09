use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemStruct};

pub fn schema_macros(item: TokenStream) -> TokenStream {
    let item_fn = syn::parse::<ItemStruct>(item).unwrap();
    let ident = item_fn.ident;
    let result = quote! {
        impl schemas::Schema for #ident {}
    };
    result.into()
}
