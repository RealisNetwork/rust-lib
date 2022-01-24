use quote::quote;
use proc_macro::TokenStream;
use syn::{self, ItemStruct};

pub fn impl_byte_encode_macros(item: TokenStream) -> TokenStream {
    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident;

    let fields = object
        .fields
        .into_iter()
        .map(|field| {
            let name = field.ident.unwrap();
            quote! {
                self.#name.encode(_byte_writer)?;
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        impl ByteSerialize for #name {
            fn encode(self, _byte_writer: &mut ByteWriter) -> Result<(), Error> {
                #(#fields)*
                Ok(())
            }
        }
    };
    gen.into()
}


