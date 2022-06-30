mod byte_decode;
mod byte_encode;
mod deserialize_errors;
mod env;
mod gettable;
mod gettable_errors;
mod retry;
mod to_json;

use proc_macro::TokenStream;

use crate::{
    byte_decode::impl_byte_decode_macros, byte_encode::impl_byte_encode_macros, deserialize_errors::impl_deserialize_errors_macros,
    env::env::impl_env_macros, gettable::impl_gettable_macros, gettable_errors::impl_gettable_errors_macros, retry::impl_retry_macros,
    to_json::impl_to_json_macros,
};

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
#[proc_macro_derive(IntoRealisErrors)]
pub fn deserialize_macro_derive_errors(input: TokenStream) -> TokenStream {
    impl_deserialize_errors_macros(input)
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

/// # Using derive
/// Provides a derive macro to generate implementations of the EnvLoaded trait
/// for data structures defined in your crate to take data from .env file by
/// fields in structure.
///
/// You only need to set this up if your code is using
///
///     #[derive(Env)]
///
/// It will search in .env file according to structure's name + "_" + field name
/// in upper case.
///
/// This functionality is based on Rust's #[derive] mechanism,
/// just like what you would use to automatically derive implementations of the
/// built-in Clone, Copy, Debug, or other traits. It is able to generate
/// implementations for most structs and enums including ones with elaborate
/// generic types or trait bounds.
///
/// Add ```realis-config = { git = "https://github.com/RealisNetwork/rust-lib.git", package = "config"}```
/// as a dependency in Cargo.toml.
///
/// On structs fields of that you want to take from env, import the derive macro
/// as
/// ```
/// use realis_config::env::EnvLoaded;
/// use realis_config::env::Env;
/// use realis_config::env::EnvLoadedError;
/// ```
/// within the same module and write #[derive(Env)] on the struct.
///
/// # Examples
/// ```
/// use realis_config::env::Env;
/// use realis_config::env::EnvLoaded;
/// use realis_config::env::EnvLoadedError;
///
/// #[derive(Debug, Env)]
/// pub struct EnvConfig {
///    pub nats: Nats,
///    pub db: Database,
///    pub runtime: Runtime,
///    pub healthchecker: Healthchecker,
///    pub job_runner: JobRunner,
///    pub logger_level: String,
/// }
/// ```
/// # Attributes
///
/// Attributes are used to customize the EnvLoaded implementations produced by
/// Env derive.
///
/// ## Field attributes
///
///     #[env(rename_abs = "...")]
/// Search the field in .env file according to the given name in upper case.
///
///     #[env(rename = "name")]
/// Search the field in .env file according to the structure's name + "_" +
/// given name in upper case.
///
///     #[env(flatten)]
/// Search the field in .env file according to the field name in upper case.
///
///     #[env(default)]
/// If the value is not present when searching in .env file, use the
/// Default::default().
///
///     #[env(default_path = "path")]
/// If the value is not present when searching in .env file, call a function to
/// get a default value. The given function must be callable as fn() -> T.
///
/// For example:
///
///  > default = "empty_value" would invoke empty_value()
///
///  > default = 'SomeTrait::some_default' would invoke
/// 'SomeTrait::some_default()'.
#[proc_macro_derive(Env, attributes(env))]
pub fn config_macro_derive(item: TokenStream) -> TokenStream {
    impl_env_macros(item)
}
