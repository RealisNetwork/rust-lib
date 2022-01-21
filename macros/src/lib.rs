use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, FnArg, Ident, ItemFn, TypeTuple, ItemStruct, __private::Span};

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
            }
            .into();
        }
        _ => panic!("Macro impl only for enums"),
    }
}

#[proc_macro_attribute]
pub fn macro_retry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse::<ItemFn>(item).unwrap();
    let original_fn_ident = item_fn.sig.ident.clone();
    let helper_fn_ident = Ident::new(&format!("{}_helper", item_fn.sig.ident.to_string()), Span::call_site());
    let params = item_fn.sig.inputs;
    let pats = params
        .clone()
        .into_iter()
        .flat_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(p) => Some(p.pat),
        })
        .collect::<Vec<_>>();
    let output = item_fn.sig.output;
    let body = item_fn.block;

    let code = quote! {
        async fn #helper_fn_ident (#params) #output {
            #body
        }

        pub async fn #original_fn_ident (#params) #output {
            retry(self.client.get_backoff(), || async { Ok(self.#helper_fn_ident(#(#pats),*).await?) }).await
        }
    };

    code.into()
}

/// # Panics
#[proc_macro_derive(ByteSerialize)]
pub fn byte_encode_macro_derive(item: TokenStream) -> TokenStream {
    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident;

    let fields = object
        .fields
        .into_iter()
        .map(|field| {
            let name = field.ident.unwrap();
            quote! {
                self.#name.encode(_byte_writer)?;
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        impl ByteSerialize for #name {
            fn encode(self, _byte_writer: &mut ByteWriter) -> Result<(), Error> {
                #(#fields)*
                Ok(())
            }
        }
    };
    gen.into()
}

/// # Panics
#[proc_macro_derive(ByteDeserialize)]
pub fn byte_decode_macro_derive(item: TokenStream) -> TokenStream {
    let object = syn::parse::<ItemStruct>(item).unwrap();
    let name = object.ident;

    let fields = object
        .fields
        .into_iter()
        .map(|field| {
            let name = field.ident.unwrap();
            let ty = field.ty;

            quote! {
                    #name: <#ty>::decode(byte_reader)?
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        impl ByteDeserialize for #name {
            fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error> {
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    };
    gen.into()
}