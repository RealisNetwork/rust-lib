use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemStruct};


pub fn impl_alivable(item: TokenStream) -> TokenStream {

    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident; // Get name of derived structure
    let hc_info = name.to_string();

    // Iterate through all fields
    let fields = object
        .fields
        .into_iter()
        .filter_map(|field| {
            let is_skiped = field.attrs.iter().any(|attribute| -> bool {
                let meta = attribute.parse_meta();
                meta.as_ref()
                    .unwrap()
                    .path()
                    .segments
                    .first()
                    .unwrap()
                    .ident
                    .to_string()
                    .contains("AliveAttr")
                    && attribute.tokens.to_string().contains("(skip)")
            });
            // If some field is not annotated as #[AliveAttr(skip)], than it will be added to
            // is_alive() method
            if !is_skiped {
                Some(field.ident.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let fields_iter = fields.iter();

    // Generate trait implementation
    let gen = quote! {
        #[async_trait]
        impl Alivable for #name {
            async fn is_alive(&self) -> bool {
                #(self.#fields_iter.is_alive().await &&)*
                true
            }

            async fn info(&self) -> &'static str {
                #hc_info
            }
        }
    };

    gen.into()
}
