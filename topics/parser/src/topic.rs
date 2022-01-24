use convert_case::{Case, Casing};
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{Ident, __private::Span};

#[derive(Debug, PartialEq)]
pub struct Topic {
    pub name: String,
    pub value: String,
}

impl FromStr for Topic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        if splitted.next().ok_or(())?.ne("export") {
            return Err(());
        }
        if splitted.next().ok_or(())?.ne("const") {
            return Err(());
        }

        let position = splitted.clone().position(|x| x == "=").ok_or(())?;

        let topic = Topic {
            name: splitted.nth(position - 1).ok_or(())?.to_string(),
            value: splitted
                .nth(1)
                .ok_or(())?
                .to_string()
                .trim_end_matches(';')
                .trim_matches('\'')
                .to_string(),
        };

        Ok(topic)
    }
}

impl ToTokens for Topic {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let name = Ident::new(&self.name.to_case(Case::UpperSnake), Span::call_site());
        let value = &self.value;

        let code = quote! {
            pub const #name: &'static str = #value;
        };

        tokens.extend(code);
    }
}

#[cfg(test)]
mod tests {
    use crate::topic::Topic;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let line = "export const AdminActionGetAllByFilterListTopic = 'admin_action_getAllByFilterList';";

        let topic = Topic::from_str(line);

        assert_eq!(
            topic,
            Ok(Topic {
                name: String::from("AdminActionGetAllByFilterListTopic"),
                value: String::from("admin_action_getAllByFilterList")
            })
        )
    }
}
