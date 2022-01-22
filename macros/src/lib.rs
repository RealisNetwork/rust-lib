mod symbol;

use quote::{ToTokens, quote};
use crate::symbol::parse_lit_into_expr_path;
use std::default::Default;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::{
    self,
    parse_macro_input,
    Data, DeriveInput,
    FnArg,
    Ident,
    ItemFn,
    TypeTuple,
    ItemStruct,
    __private::Span,
    Meta::{NameValue, Path},
    NestedMeta::Meta,
    Lit::Str,
    Type,
    Lit,
};

/// # Panics
#[proc_macro_derive(Gettable, attributes(gettable))]
pub fn gettable_macro_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    impl_gettable_macro(&ast, &attr)
}

fn impl_gettable_macro(ast: &DeriveInput, attr: &TypeTuple) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];

    let gen = quote! {
        use convert_case::{Case, Casing};

        impl Gettable for #name {
            type Error = #error;
            type MessageParams = #params;
            type MessageReturn = #returns;

            fn topic() -> String {
                #name_string.to_case(Case::Snake)
            }

            fn parse(payload: &[u8]) -> Result<Box<
            dyn Message<Params=Self::MessageParams, Return=Self::MessageReturn>
            >, Self::Error> {
                let message = serde_json::from_slice::<#name>(payload)?;
                Ok(Box::new(message))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ToJson)]
pub fn to_json_macro_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident;
    quote! { impl ToJson for #name {} }.into()
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    match data {
        Data::Enum(data_enum) => {
            let cases = data_enum
                .variants
                .iter()
                .map(|variant| variant.ident.clone())
                .map(|variant| {
                    let variant_name = variant.to_string().to_case(Case::Camel);
                    quote! {#name::#variant(value) => format!("{}.{}", #variant_name, value.as_string() )}
                })
                .collect::<Vec<_>>();

            return quote! {
                impl ToJson for #name {
                    fn as_string(&self) -> String {
                        match self {
                            #(#cases),*
                        }
                    }
                }
            }
            .into();
        }
        _ => panic!("Macro impl only for enums"),
    }
}

#[proc_macro_attribute]
pub fn macro_retry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse::<ItemFn>(item).unwrap();
    let original_fn_ident = item_fn.sig.ident.clone();
    let helper_fn_ident = Ident::new(&format!("{}_helper", item_fn.sig.ident.to_string()), Span::call_site());
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

/// # Panics
#[proc_macro_derive(ByteSerialize)]
pub fn byte_encode_macro_derive(item: TokenStream) -> TokenStream {
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

/// # Panics
#[proc_macro_derive(ByteDeserialize)]
pub fn byte_decode_macro_derive(item: TokenStream) -> TokenStream {
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

#[proc_macro_derive(Env, attributes(env))]
pub fn config_macro_derive(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse::<ItemStruct>(item).unwrap();
    let struct_name = item_struct.ident;

    let fields = item_struct.fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.unwrap().to_token_stream();

            let mut field_attributes = Field::default();

            field
                .attrs
                .into_iter()
                .map(|attr| symbol::get_env_meta_items(&attr))
                .flatten()
                .for_each(|meta| {
                    match meta {
                        Meta(NameValue(m)) if m.path == symbol::RENAME => field_attributes.field_type = EnvRenameAttrs::Rename(m.lit),
                        Meta(Path(path)) if path == symbol::FLATTEN => field_attributes.field_type = EnvRenameAttrs::Flatten(field_name.to_string()),
                        Meta(NameValue(m)) if m.path == symbol::RENAME_ABS => field_attributes.field_type = EnvRenameAttrs::RenameAbs(m.lit),
                        Meta(Path(path)) if path == symbol::DEFAULT => field_attributes.default_type = EnvDefaultAttrs::Default,
                        Meta(NameValue(m)) if m.path == symbol::DEFAULT_PATH => field_attributes.default_type = EnvDefaultAttrs::DefaultPath(m.lit),
                        _ => {},
                    }
                });

            let env_name = match field_attributes.field_type {
                EnvRenameAttrs::Rename(Str(str)) => format!("{}_{}", struct_name.to_string(), str.value()).to_case(Case::UpperSnake),
                EnvRenameAttrs::RenameAbs(Str(str)) => format!("{}", str.value()).to_case(Case::UpperSnake),
                EnvRenameAttrs::Flatten(_) => format!("{}", field_name).to_case(Case::UpperSnake),
                _ => format!("{}_{}", struct_name.to_string(), field_name.to_string()).to_case(Case::UpperSnake)
            };

            match (field_attributes.default_type, field.ty) {
                (EnvDefaultAttrs::Default, Type::Path(field_type)) => {
                    quote! {
                        #field_name: #field_type::load(#env_name).unwrap_or_default(),
                    }
                },
                (EnvDefaultAttrs::DefaultPath(lit), Type::Path(field_type)) => {
                    let path = parse_lit_into_expr_path(&lit);
                    quote! {
                        #field_name: #field_type::load(#env_name).unwrap_or(#path()),
                    }
                },
                (EnvDefaultAttrs::Empty, Type::Path(field_type)) => {
                    quote! {
                        #field_name: #field_type::load(#env_name)?,
                    }
                },
                (_, field_type) => panic!("Env don't support this type: {:?}", field_type),
            }
        });

    let code = quote! {
        impl EnvLoaded for #struct_name {
            fn load(_key: &str) -> Result<Self, EnvLoadedError> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };

    code.into()
}

#[derive(PartialEq)]
enum EnvRenameAttrs {
    Flatten(String),
    RenameAbs(Lit),
    Rename(Lit),
    Empty,
}

enum EnvDefaultAttrs {
    Default,
    DefaultPath(Lit),
    Empty,
}

struct Field {
    field_type: EnvRenameAttrs,
    default_type: EnvDefaultAttrs,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_type: EnvRenameAttrs::Empty,
            default_type: EnvDefaultAttrs::Empty,
        }
    }
}
