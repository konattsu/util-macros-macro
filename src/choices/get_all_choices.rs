use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::inner_type::ChoicesItemEnum;

pub(super) fn get_all_choices_quote(
    item_enum: &ChoicesItemEnum,
    case_sensitive: bool,
) -> TokenStream2 {
    let choices: Vec<TokenStream2> = item_enum
        .variants
        .iter()
        .map(|variant| {
            let name = variant.ident.to_string();
            let name = if case_sensitive {
                name
            } else {
                name.to_lowercase()
            };
            quote! { #name }
        })
        .collect();
    quote! {
        fn get_all_choices() -> Vec<String> {
            vec![#(#choices.to_string()),*]
        }
    }
}
