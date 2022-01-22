use quote::{ToTokens, quote};
use crate::symbol::parse_lit_into_expr_path;
use std::default::Default;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::{self, parse_macro_input, Data, DeriveInput, FnArg, Ident, ItemFn, TypeTuple, ItemStruct, __private::Span, Meta::{NameValue, Path}, NestedMeta::Meta, Lit::Str, Type, Lit};
use crate::structs::{Field, EnvRenameAttrs, EnvDefaultAttrs};
use crate::symbol;

pub fn impl_env_macros(item: TokenStream) -> TokenStream {
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
                (_, field_type) => panic!("Env don't support this type"),
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
