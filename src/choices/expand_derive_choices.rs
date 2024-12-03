use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemEnum;

use super::from_str::from_str_quote;
use super::get_all_choices::get_all_choices_quote;
use super::inner_type::Choices;

pub(crate) fn expand_derive_choices(
    attr: TokenStream,
    item: TokenStream,
) -> syn::Result<TokenStream> {
    // 欲しい型を見つけるには引数の`TokenStream`を`dbg!`でデバッグ出力
    // その後適する型を見つける(頑張る)
    // ちなみに `as` はキャスト変換ではなく変換先の型を指定している
    //
    // 型が無ければ (基本的に`attr`から変換するとき) 自作の構造体へ変換すると良い
    // - `syn::parse::Parse` を実装すること
    // - 実装には`i.parse()?`が便利

    let choices = Choices::new(attr, item)?;
    let item_enum = choices.get_item_enum();
    let case_sensitive = choices.get_case_sensitive();
    let enum_name = choices.get_enum_name();

    let from_str_quote: TokenStream2 = from_str_quote(&choices);
    let all_choices_quote: TokenStream2 =
        get_all_choices_quote(&item_enum, case_sensitive);
    let trait_impl: TokenStream2 = quote! {
        #from_str_quote
        impl util_macros_core::choices::Choices for #enum_name {
            #all_choices_quote
        }
    };
    let item_enum: ItemEnum = item_enum.into();

    let res = quote! {
        #item_enum
        #trait_impl
    };

    Ok(TokenStream::from(res))
}
