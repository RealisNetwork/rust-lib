use quote::__private::TokenStream;
use quote::{quote, ToTokens};

pub struct SchemaDeclaration {
    pub declaration: TokenStream,
    pub prefix: TokenStream,
    pub contains_struct: bool,
}

impl Default for SchemaDeclaration {
    fn default() -> Self {
        SchemaDeclaration {
            declaration: quote! {},
            prefix: quote! {},
            contains_struct: false,
        }
    }
}

impl ToTokens for SchemaDeclaration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.prefix.clone());
        tokens.extend(self.declaration.clone());
    }
}
