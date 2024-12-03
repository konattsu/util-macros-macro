#[cfg(feature = "choices")]
mod choices;

#[allow(unused)]
use proc_macro::TokenStream;

/// Treat enum as a `choice`
///
/// This macro automatically adds the following functionality to an enumerated type.
///
/// - Implement the `from_str` method to create an enumerated type from a string
/// - Implement the `get_all_choices` method that returns all values contained in the enumerated type
///
/// This macro is only available for enumerated types that **do not** have any internal values(i.e. size:0x1).
///
/// Attribute arguments
/// - bool: whether case-sensitive
#[proc_macro_attribute]
#[cfg(feature = "choices")]
pub fn choices(args: TokenStream, item: TokenStream) -> TokenStream {
    choices::expand_derive_choices(args, item)
        .unwrap_or_else(|a| syn::Error::into_compile_error(a).into())
}
