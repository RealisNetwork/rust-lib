use crate::syn::Error;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{self, parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, TypeTuple};
use serde_json::{Value, json};

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

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string).to_compile_error().into();
    };
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(item: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    match data {
        // Only if data is an enum, we do parsing
        Data::Enum(data_enum) => {
            // data_enum is of type syn::DataEnum
            // https://doc.servo.org/syn/struct.DataEnum.html

            variant_checker_functions = TokenStream2::new();

            // Iterate over enum variants
            // `variants` if of type `Punctuated` which implements IntoIterator
            //
            // https://doc.servo.org/syn/punctuated/struct.Punctuated.html
            // https://doc.servo.org/syn/struct.Variant.html
            for variant in &data_enum.variants {
                // Variant's name
                let ref variant_name = variant.ident;

                // Variant can have unnamed fields like `Variant(i32, i64)`
                // Variant can have named fields like `Variant {x: i32, y: i32}`
                // Variant can be named Unit like `Variant`
                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };
                let expanded = quote! {
                    impl #name {
                        pub fn enum_to_string(&self) -> String {
                            match self {
                               #name::#variant_name(_) => format!("{}.{}", stringify!(#name), stringify!(#variant_name(_))),
                               //#name::#variant_name => format!("{}.{}", stringify!(#name), stringify!(#variant_name))
                            }
                        }

                        pub fn error_to_value_with_message(&self, msg: &str) -> Value {
                            match self {
                                #name::#variant_name(_) => {
                                    return json!({
                                        "type": "Left",
                                        "value": {
                                            "msg": msg,
                                            "type": format!("{}.{}", stringify!(#name), stringify!(#variant_name(_)))
                                        }
                                    })
                                },
                                // #name::#variant_name => {
                                //     return json!({
                                //         "type": "Left",
                                //         "value": {
                                //             "msg": msg,
                                //             "type": format!("{}.{}", stringify!(#name), stringify!(#variant_name))
                                //         }
                                //     })
                                // }
                            }
                        }

                        pub fn error_to_value(&self) -> Value {
                            match self {
                                #name::#variant_name(_) => {
                                    return json!({
                                        "type": "Left",
                                        "value": {
                                            "type": format!("{}.{}", stringify!(#name), stringify!(#variant_name(_)))
                                        }
                                    })
                                },
                                // #name::#variant_name => {
                                //     return json!({
                                //         "type": "Left",
                                //         "value": {
                                //             "type": format!("{}.{}", stringify!(#name), stringify!(#variant_name))
                                //         }
                                //     })
                                // }
                            }
                        }
                        }
                };
                return TokenStream::from(expanded);
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };
    return TokenStream::default()
}

// fn impl_gettable_macro_errors(ast: &DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let attr = &ast.data;
//     match attr {
//         Data::Enum(data_enum) => {
//             let gen = quote! {
//                 use serde_json::json;
//                 use serde_json::Value;
//         impl #name {
//             pub fn enum_to_string(&self) -> String {
//                 match self {
//                    #name::attr => format!("{}.{}", stringify!(#name),
// stringify!(data_enum.variants))                 }
//             }
//
//             pub fn error_to_value_with_message(&self, msg: &str) -> Value {
//                 match self {
//                             #name::attr => {
//                         return json!({
//                             "type": "Left",
//                             "value": {
//                                 "msg": msg,
//                                 "type": format!("{}.{}", stringify!(#name),
// stringify!(data_enum.variants))                             }
//                         })
//                     }
//                 }
//             }
//
//             pub fn error_to_value(&self) -> Value {
//                 match self {
//                             #name::attr => {
//                         return json!({
//                             "type": "Left",
//                             "value": {
//                                 "type": format!("{}.{}", stringify!(#name),
// stringify!(data_enum.variants))                             }
//                         })
//                     }
//                 }
//             }
//         };
//     };
//             gen.into()
//         }
//         _ => TokenStream::default()
//     }
// }
