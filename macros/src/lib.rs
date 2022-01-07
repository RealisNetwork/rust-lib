use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, TypeTuple};

/// # Panics
#[proc_macro_derive(Gettable, attributes(gettable))]
pub fn gettable_macro_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let attr = syn::parse::<TypeTuple>(ast.attrs[0].tokens.clone().into()).unwrap();

    impl_gettable_macro(&ast, &attr)
}

fn impl_gettable_macro(ast: &DeriveInput, attr: &TypeTuple) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();

    let error = &attr.elems[0];
    let params = &attr.elems[1];
    let returns = &attr.elems[2];

    let gen = quote! {
        use convert_case::{Case, Casing};

        impl Gettable for #name {
            type Error = #error;
            type MessageParams = #params;
            type MessageReturn = #returns;

            fn topic() -> String {
                #name_string.to_case(Case::Snake)
            }

            fn parse(payload: &[u8]) -> Result<Box<
            dyn Message<Params=Self::MessageParams, Return=Self::MessageReturn>
            >, Self::Error> {
                let message = serde_json::from_slice::<#name>(payload)?;
                Ok(Box::new(message))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ToJson)]
pub fn to_json_macro_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident;
    quote! { impl ToJson for #name {} }.into()
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

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

            return quote! {
                impl ToJson for #name {
                    fn as_string(&self) -> String {
                        match self {
                            #(#cases),*
                        }
                    }
                }
            }.into()

        },
        _ => panic!("Macro impl only for enums")
    }
}
