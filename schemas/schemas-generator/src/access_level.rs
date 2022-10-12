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
        // I remove redundant staff in agent
        // not bad, i think
        // what do you mean?
        // that we have so much errors :D
        // try to rebuild or clean rebuild
        // i rebuilded at now
        tokens.extend(t);
    }
}
