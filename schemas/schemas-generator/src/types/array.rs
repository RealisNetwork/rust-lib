use crate::agent_params::AgentParams;
use crate::schema_declaration::SchemaDeclaration;
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
        let declaration = quote! { pub type #name_ident = #parameter_type; };
        (prefix, declaration)
    }

    pub fn get_type(&self, name: &str) -> TokenStream {
        let parameter_type = self.parameter.get_type(&format!("{}Params", name));
        quote! { Vec<#parameter_type> }
    }

    pub fn contains_struct(&self) -> bool {
        match &*self.parameter {
            AgentParams::Array(array) => array.contains_struct(),
            AgentParams::Object(_) => true,
            _ => false,
        }
    }

    pub fn get_schema_declaration(&self, name: &str) -> SchemaDeclaration {
        let (prefix, declaration) = self.get_declaration(name);
        SchemaDeclaration {
            declaration,
            prefix,
            contains_struct: self.contains_struct(),
        }
    }
}
