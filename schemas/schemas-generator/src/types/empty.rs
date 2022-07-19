use quote::__private::{Ident, TokenStream};
use quote::quote;
use syn::__private::Span;

pub struct Empty;

impl Empty {
    pub fn get_declaration(name: &str) -> (TokenStream, TokenStream) {
        let ident = Ident::new(name, Span::call_site());
        (
            quote! {
                impl<'de> Deserialize<'de> for #ident {
                    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        Ok(#ident)
                    }
                }
            },
            quote! {
                #[derive(Debug, Clone, Serialize)]
                pub struct #ident;
            },
        )
    }

    pub fn get_type() -> TokenStream {
        quote! { () }
    }

    pub fn get_schema_declaration(name: &str) -> TokenStream {
        let (mut prefix, declaration) = Self::get_declaration(name);
        prefix.extend(declaration);
        prefix
    }
}
