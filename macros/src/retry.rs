use proc_macro::TokenStream;
use quote::quote;
use syn::{self, FnArg, Ident, ItemFn, __private::Span};

pub fn impl_retry_macros(item: TokenStream) -> TokenStream {
    let item_fn = syn::parse::<ItemFn>(item).unwrap();
    let original_fn_ident = item_fn.sig.ident.clone();
    let helper_fn_ident = Ident::new(&format!("{}_helper", item_fn.sig.ident), Span::call_site());
    let params = item_fn.sig.inputs;
    let pats = params
        .clone()
        .into_iter()
        .flat_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(p) => Some(p.pat),
        })
        .collect::<Vec<_>>();
    let output = item_fn.sig.output;
    let body = item_fn.block;

    let code = quote! {
        async fn #helper_fn_ident (#params) #output {
            #body
        }

        pub async fn #original_fn_ident (#params) #output {
            retry(self.client.get_backoff(), || async { Ok(self.#helper_fn_ident(#(#pats),*).await?) }).await
        }
    };

    code.into()
}
