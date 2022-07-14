use proc_macro::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{self, Attribute, DeriveInput, Fields, Item, ItemStruct, TypeGroup, TypeTuple};

pub fn impl_alivable(item: TokenStream) -> TokenStream {
    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident;

    let fields = object
        .fields
        .into_iter()
        .filter_map(|field| {
            let is_skiped = field
                .attrs
                .iter()
                .find(|attribute| -> bool {
                    let meta = attribute.parse_meta();
                    meta.as_ref()
                        .unwrap()
                        .path()
                        .segments
                        .first()
                        .unwrap()
                        .ident
                        .to_string()
                        .find("AliveAttr")
                        .is_some()
                        && attribute.tokens.to_string().find("(skip)").is_some()
                })
                .is_some();
            if !is_skiped {
                Some(field.ident.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let fields_iter = fields.iter();

    let gen = quote! {
        #[async_trait]
        impl Alivable for #name {
            async fn is_alive(&self) -> bool {
                #(self.#fields_iter.is_alive().await &&)*
                true
            }

            async fn info(&self) -> &'static str {
                "#name"
            }
        }
    };

    gen.into()
}
