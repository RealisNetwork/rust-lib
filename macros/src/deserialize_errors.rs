use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, ItemEnum};

pub fn impl_deserialize_errors_macros(input: TokenStream) -> TokenStream {
    let input = syn::parse::<ItemEnum>(input).unwrap();
    let name = input.ident;
    let mut data = input.variants;
    data.pop();
    data.pop();
    let expr = data
        .into_iter()
        .map(|variant| {
            let ident = variant.ident;
            let ident_str = &ident.to_string();
            quote! {
                    #ident_str => #name::#ident(#ident::from_str(&enum_element)?),
            }
        })
        .collect::<Vec<TokenStream2>>();

    let gen = quote! {
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let error = String::deserialize(deserializer)?;

                if error.contains('.') {
                    let splitted = error.split('.').map(String::from).collect::<Vec<String>>();
                    let enum_name = splitted
                        .first()
                        .ok_or(D::Error::custom("Missing error prefix error!"))?
                        .to_case(Case::UpperCamel);
                    let element = splitted
                        .get(1)
                        .ok_or(D::Error::custom("Missing error postfix error!"))?;

                    let realis_error = match match_error(enum_name, element) {
                        Ok(realis_err) => realis_err,
                        Err(_) => RealisErrors::Common(Common::Unknown),
                    };
                    Ok(realis_error)
                } else {
                    Err(D::Error::custom("Unknown error!"))
                }
            }
        }

        pub fn match_error(enum_name: String, element: &String) -> Result<RealisErrors, ParseError> {
            let enum_element = element.to_case(Case::UpperCamel);
            let realis_error = match enum_name.as_str() {
                        #(#expr)*
                        "CustomInt" => #name::CustomInt(element.clone().parse::<i32>().expect("Not a number!")),
                        "CustomString" => #name::CustomString(element.clone()),
                        _ => RealisErrors::Common(Common::Unknown),
                    };
            Ok(realis_error)
        }
    };

    gen.into()
}