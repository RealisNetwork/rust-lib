use crate::agent_params::AgentParams;
use quote::__private::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::__private::Span;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Array {
    pub parameter: Box<AgentParams>,
}

impl Array {
    pub fn get_declaration(&self, name: &str) -> (TokenStream, TokenStream) {
        let (prefix, declaration) = self.parameter.get_declaration(&format!("{}Params", name));
        let prefix = match *self.parameter {
            AgentParams::Array(_) => prefix,
            AgentParams::Object(_) => quote! {
                #prefix
                #declaration
            },
            _ => quote! {},
        };

        let parameter_type = self.get_type(name);
        let name_ident = Ident::new(name, Span::call_site());
        let declaration = quote! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct #name_ident(pub #parameter_type);
        };
        (prefix, declaration)
    }

    pub fn get_type(&self, name: &str) -> TokenStream {
        let parameter_type = self.parameter.get_type(&format!("{}Params", name));
        quote! { Vec<#parameter_type> }
    }

    pub fn get_schema_declaration(&self, name: &str) -> TokenStream {
        let (mut prefix, declaration) = self.get_declaration(name);
        prefix.extend(declaration);
        prefix
    }
}
