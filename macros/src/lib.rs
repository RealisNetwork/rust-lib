mod symbol;
mod structs;
mod env;
mod gettable;
mod gettable_errors;
mod retry;
mod byte_encode;
mod byte_decode;
mod to_json;

use proc_macro::TokenStream;
use crate::byte_decode::impl_byte_decode_macros;
use crate::byte_encode::impl_byte_encode_macros;
use crate::env::impl_env_macros;
use crate::gettable::impl_gettable_macros;
use crate::gettable_errors::impl_gettable_errors_macros;
use crate::retry::impl_retry_macros;
use crate::to_json::impl_to_json_macros;

/// # Panics
#[proc_macro_derive(Gettable, attributes(gettable))]
pub fn gettable_macro_derive(item: TokenStream) -> TokenStream {
    impl_gettable_macros(item)
}

#[proc_macro_derive(ToJson)]
pub fn to_json_macro_derive(item: TokenStream) -> TokenStream {
    impl_to_json_macros(item)
}

/// # Panics
#[proc_macro_derive(RealisErrors)]
pub fn gettable_macro_derive_errors(input: TokenStream) -> TokenStream {
    impl_gettable_errors_macros(input)
}

#[proc_macro_attribute]
pub fn macro_retry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_retry_macros(item)
}

/// # Panics
#[proc_macro_derive(ByteSerialize)]
pub fn byte_encode_macro_derive(item: TokenStream) -> TokenStream {
    impl_byte_encode_macros(item)
}

/// # Panics
#[proc_macro_derive(ByteDeserialize)]
pub fn byte_decode_macro_derive(item: TokenStream) -> TokenStream {
    impl_byte_decode_macros(item)
}

#[proc_macro_derive(Env, attributes(env))]
pub fn config_macro_derive(item: TokenStream) -> TokenStream {
    impl_env_macros(item)
}
