use crate::agents::Agent;
use quote::__private::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct AgentsGenerator {
    pub agents: Agent,
}

impl AgentsGenerator {
    fn get_variants_names(&self) -> Ident {
        self.agents.create_ident()
    }

    // fn impl_from_string(&self) -> TokenStream {
    //     let from_string = self
    //         .agents
    //         .iter()
    //         .map(|agent| {
    //             let agent_name = &agent.agent;
    //             let method = &agent.method;
    //             let ident = agent.create_ident();

    //             quote! { (#agent_name, #method) => Some(AbstractSchema::#ident), }
    //         })
    //         .collect::<Vec<_>>();

    //     quote! {
    //         pub fn from_string(agent: &str, method: &str) -> Option<Self> {
    //             match (agent, method) {
    //                 #(#from_string)*
    //                 _ => None,
    //             }
    //         }
    //     }
    // }
}

impl ToTokens for AgentsGenerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let imports = quote! {
            use serde::{Serialize, Deserialize};
        };

        let agents = self.agents.clone().into_token_stream();

        let agents_declaration = quote! {
            #agents
        };

        // let variants_names = self.get_variants_names();

        // let declaration = quote! {
        //     pub enum AbstractSchema {
        //         #(#variants_names),*
        //     }
        // };

        // let from_string = self.impl_from_string();

        // let implementation = quote! {
        //     impl AbstractSchema {
        //         #from_string
        //     }
        // };

        let stream = quote! {
            #![allow(unknown_lints)]
            #![allow(clippy::all)]

            #imports

            #agents_declaration

            // #declaration

            // #implementation

        };

        tokens.extend(stream);
    }
}
