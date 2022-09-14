use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput};

pub fn impl_gettable_errors_macros(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let name = &input.ident;
    let data = &input.data;

    match data {
        Data::Enum(data_enum) => {
            let cases = data_enum
                .variants
                .iter()
                .map(|variant| variant.ident.clone())
                .map(|variant| {
                    let variant_name = variant.to_string().to_case(Case::Camel);
                    quote! {#name::#variant(value) => format!("{}.{}", #variant_name, value.as_string() )}
                })
                .collect::<Vec<_>>();

            quote! {
                impl ToJson for #name {
                    fn as_string(&self) -> String {
                        match self {
                            #(#cases),*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("Macro impl only for enums"),
    }
}
