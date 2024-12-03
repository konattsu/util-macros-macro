use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::inner_type::Choices;

pub(super) fn from_str_quote(choices: &Choices) -> TokenStream2 {
    let item_enum = choices.get_item_enum();
    let case_sensitive = choices.get_case_sensitive();
    let enum_name = choices.get_enum_name();

    let match_arms = item_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let name = &variant.ident.to_string();
        let name = if case_sensitive {
            name
        } else {
            &name.to_lowercase()
        };
        quote! {
            #name => Ok(#enum_name::#ident),
        }
    });

    let lowercase_conversion = if case_sensitive {
        quote! { s }
    } else {
        quote! { s.to_lowercase().as_str() }
    };
    quote! {
        impl std::str::FromStr for #enum_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match #lowercase_conversion {
                    #(#match_arms)*
                    _ => Err(format!("Invalid input:`{}`", s)),
                }
            }
        }
    }
}
