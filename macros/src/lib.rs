use crate::syn::Error;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use serde_json::{json, Value};
use syn::{self, parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, TypeTuple};

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

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string).to_compile_error().into();
    };
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            let cases = data_enum
                .variants
                .iter()
                .map(|variant| variant.ident.clone())
                .map(|variant| {
                    let variant_name = variant.to_string().to_case(Case::Camel);
                    quote! {#name::#variant(value) => format!("{}.{}", #variant_name, value)}
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

                // construct an identifier named is_<variant_name> for function name
                // We convert it to snake case using `to_case(Case::Snake)`
                // For example, if variant is `HelloWorld`, it will generate `is_hello_world`
                let mut error_to_value_with_message =
                    format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));
                error_to_value_with_message.set_span(variant_name.span());

                // Here we construct the function for the current variant
                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    pub fn #error_to_value_with_message(&self, msg: String) -> serde_json::Value {
                        match self {
                            #name::#variant_name #fields_in_variant => return json!({
                            "type": "Left",
                            "value": {
                                "msg": msg,
                                "type": format!("{}.{}", stringify!(#variant_name),stringify!(#fields_in_variant))
                                    }
                        }),
                            _ => return json!({
                            "type": "Left",
                            "value": {
                                "msg": msg,
                                "type": format!("{}.{}", stringify!(#name),stringify!(#variant_name))
                                    }
                        }),
                        }
                    }
                });

                // Above we are making a TokenStream using extend()
                // This is because TokenStream is an Iterator,
                // so we can keep extending it.
                //
                // proc_macro2::TokenStream:- https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.TokenStream.html

                // Read about
                // quote:- https://docs.rs/quote/1.0.7/quote/
                // quote_spanned:- https://docs.rs/quote/1.0.7/quote/macro.quote_spanned.html
                // spans:- https://docs.rs/syn/1.0.54/syn/spanned/index.html
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #name {
            // variant_checker_functions gets replaced by all the functions
            // that were constructed above
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
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
