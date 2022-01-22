use quote::quote;
use proc_macro::TokenStream;
use syn::{self, ItemStruct};

pub fn impl_byte_decode_macros(item: TokenStream) -> TokenStream {
    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident;

    let fields = object
        .fields
        .into_iter()
        .map(|field| {
            let name = field.ident.unwrap();
            let ty = field.ty;

            quote! {
                    #name: <#ty>::decode(byte_reader)?
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        impl ByteDeserialize for #name {
            fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error> {
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    };
    gen.into()
}
