use crate::env::{
    field_attributes::{EnvDefaultAttrs, EnvNameAttrs, Field},
    symbol,
};
use proc_macro::TokenStream;
use quote::quote;
use std::default::Default;
use syn::{
    self, ItemStruct,
    Meta::{NameValue, Path},
    NestedMeta::Meta,
    Type,
};

pub fn impl_env_macros(item: TokenStream) -> TokenStream {
    let item_struct = syn::parse::<ItemStruct>(item).unwrap();
    let struct_name = item_struct.ident;

    let fields = item_struct.fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();

        let mut field_attributes = Field::default();

        field
            .attrs
            .into_iter()
            .flat_map(|attr| symbol::get_env_meta_items(&attr))
            // .map(|attr| symbol::get_env_meta_items(&attr))
            // .flatten()
            .for_each(|meta| match meta {
                Meta(NameValue(m)) if m.path == symbol::RENAME => {
                    field_attributes.field_name = EnvNameAttrs::Rename(m.lit)
                }
                Meta(Path(path)) if path == symbol::FLATTEN => {
                    field_attributes.field_name = EnvNameAttrs::Flatten
                }
                Meta(NameValue(m)) if m.path == symbol::RENAME_ABS => {
                    field_attributes.field_name = EnvNameAttrs::RenameAbs(m.lit)
                }
                Meta(Path(path)) if path == symbol::DEFAULT => {
                    field_attributes.default_type = EnvDefaultAttrs::Default
                }
                Meta(NameValue(m)) if m.path == symbol::DEFAULT_PATH => {
                    field_attributes.default_type = EnvDefaultAttrs::DefaultPath(m.lit)
                }
                _ => {}
            });

        let env_name = field_attributes
            .field_name
            .name(&struct_name.to_string(), &field_name.to_string());
        let postfix = field_attributes.default_type.postfix();

        match field.ty {
            Type::Path(field_type) => {
                quote! {
                    #field_name: #field_type::load(Some(#env_name.to_string()))#postfix,
                }
            }
            field_type => panic!("Env don't support this type: {:?}", field_type),
        }
    });

    let code = quote! {
        impl EnvLoaded for #struct_name {
            fn load(_key: Option<String>) -> Result<Self, BaseError<()>> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };

    code.into()
}
