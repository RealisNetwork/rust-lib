use quote::__private::TokenStream;

pub trait Parameterizable {
    /// Returns prefix and parameter type
    /// parameter - Rust type or struct name? which was declared in prefix
    /// prefix - Declaration of struct or emtpy for primitive types
    fn get_type(&self, name: &str) -> (TokenStream, TokenStream);

    /// Returns way in which this parameter must be read from bytes
    /// This method suppose that there is `byte_reader` in the scope
    /// and locate decoded parameter in variable with name `params`
    fn impl_read_from_bytes(&self, name: &str) -> TokenStream;

    /// Returns way in which this parameter must be write to bytes
    /// This method suppose that there is `byte_writer` in the scope
    /// and parameter located in variable with name `object`
    fn impl_write_to_bytes(&self) -> TokenStream;
}
