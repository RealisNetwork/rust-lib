use crate::parser::schema::Schema;
use quote::{__private::TokenStream, quote, ToTokens};
use syn::Ident;

pub struct SchemasGenerator {
    pub schemas: Vec<Schema>,
}

impl SchemasGenerator {
    fn get_variants_names(&self) -> Vec<Ident> {
        self.schemas.iter().map(Schema::create_ident).collect()
    }

    fn impl_from_bytes(&self) -> TokenStream {
        let from_bytes = self
            .schemas
            .iter()
            .map(|schema| {
                let byte_agent = schema.byte_agent;
                let byte_method = schema.byte_method;
                let ident = schema.create_ident();

                quote! { (#byte_agent, #byte_method) => Some(AbstractSchema::#ident), }
            })
            .collect::<Vec<_>>();

        quote! {
            pub fn from_bytes(agent: u8, method: u8) -> Option<Self> {
                match (agent, method) {
                    #(#from_bytes)*
                    _ => None,
                }
            }
        }
    }

    fn impl_from_string(&self) -> TokenStream {
        let from_string = self
            .schemas
            .iter()
            .map(|schema| {
                let agent = &schema.agent;
                let method = &schema.method;
                let ident = schema.create_ident();

                quote! { (#agent, #method) => Some(AbstractSchema::#ident), }
            })
            .collect::<Vec<_>>();

        quote! {
            pub fn from_string(agent: &str, method: &str) -> Option<Self> {
                match (agent, method) {
                    #(#from_string)*
                    _ => None,
                }
            }
        }
    }

    fn impl_get_bytes_method(&self) -> TokenStream {
        let variants_names = self.get_variants_names();

        let case = variants_names
            .into_iter()
            .zip(self.schemas.clone())
            .map(|(name, schema)| {
                let agent = schema.byte_agent;
                let method = schema.byte_method;
                quote! {
                    AbstractSchema::#name => (#agent, #method),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn get_bytes_method(&self) -> (u8, u8) {
                match self {
                    #(#case)*
                }
            }
        }
    }

    fn impl_get_method(&self) -> TokenStream {
        let case = self
            .schemas
            .iter()
            .map(|schema| {
                let agent = &schema.agent;
                let method = &schema.method;
                let name = schema.create_ident();
                quote! {
                    AbstractSchema::#name => (String::from(#agent), String::from(#method)),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn get_method(&self) -> (String, String) {
                match self {
                    #(#case)*
                }
            }
        }
    }

    fn impl_into_json(&self) -> TokenStream {
        let variants_names = self.get_variants_names();

        let expr = variants_names
            .into_iter()
            .map(|name| {
                quote! {
                    AbstractSchema::#name => #name::from_bytes_to_json(byte_reader),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn into_json(self, byte_reader: &mut ByteReader) -> Result<Value, byte_formatter::Error> {
                match self {
                    #(#expr)*
                }
            }
        }
    }

    fn impl_into_bytes(&self) -> TokenStream {
        let variant_names = self.get_variants_names();

        let cases = variant_names
            .into_iter()
            .map(|variant_name| {
                quote! {
                    AbstractSchema::#variant_name => #variant_name::from_value_to_bytes(value),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn into_bytes(self, value: Value) -> Result<Vec<u8>, EncodeError> {
                match self {
                    #(#cases)*
                }
            }
        }
    }

    fn impl_topic_to_send(&self) -> TokenStream {
        let variant_names = self.get_variants_names();

        let cases = variant_names
            .into_iter()
            .map(|variant_name| {
                quote! {
                    AbstractSchema::#variant_name => #variant_name::topic_to_send(),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn topic_to_send(&self) -> String {
                match self {
                    #(#cases)*
                }
            }
        }
    }

    fn impl_topic_to_response(&self) -> TokenStream {
        let variant_names = self.get_variants_names();

        let cases = variant_names
            .into_iter()
            .map(|variant_name| {
                quote! {
                    AbstractSchema::#variant_name => #variant_name::topic_to_response(),
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn topic_to_response(&self) -> String {
                match self {
                    #(#cases)*
                }
            }
        }
    }
}

impl ToTokens for SchemasGenerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let imports = quote! {
            use byte_formatter::{ByteReader, ByteWriter, ByteSerialize, ByteDeserialize, Error};
            pub use crate::parser::traits::{Convertable, EncodeError};
            use serde_json::Value;
            use serde::{Serialize, Deserialize};
        };

        let schemas = self
            .schemas
            .iter()
            .map(quote::ToTokens::into_token_stream)
            .collect::<Vec<TokenStream>>();

        let schemas_declaration = quote! {
            #(#schemas)*
        };

        let variants_names = self.get_variants_names();

        let declaration = quote! {
            pub enum AbstractSchema {
                #(#variants_names),*
            }
        };

        ////

        let from_bytes = self.impl_from_bytes();
        let from_string = self.impl_from_string();
        let get_bytes_method = self.impl_get_bytes_method();
        let get_method = self.impl_get_method();
        let into_json = self.impl_into_json();
        let into_bytes = self.impl_into_bytes();
        let topic_to_send = self.impl_topic_to_send();
        let topic_to_response = self.impl_topic_to_response();

        let implementation = quote! {
            impl AbstractSchema {
                #from_bytes
                #from_string
                #get_bytes_method
                #get_method
                #into_json
                #into_bytes
                #topic_to_send
                #topic_to_response
            }
        };

        // TODO: Check this

        let stream = quote! {

            //#![allow(unknown_lints)]
            //#![allow(clippy::all)]
            #imports

            #schemas_declaration

            //#declaration

            //#implementation

        };

        tokens.extend(stream);
    }
}
