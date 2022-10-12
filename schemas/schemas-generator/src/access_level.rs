use quote::__private::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)] // Add copy because this is simple u8... )
#[serde(rename_all = "snake_case")] // why snake_case?? maybe camelCase??
pub enum AccessLevel {
    Public,
    Protected,
    Private,
    Internal,
}

impl ToTokens for AccessLevel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let t = match self {
            Self::Public => quote! {AccessLevel::Public},
            Self::Protected => quote! {AccessLevel::Protected},
            Self::Private => quote! {AccessLevel::Private},
            Self::Internal => quote! {AccessLevel::Internal},
        };
        tokens.extend(t);
    }
}
