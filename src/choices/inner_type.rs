use std::ops::{Deref, DerefMut};

use proc_macro::TokenStream;
use syn::{ItemEnum, LitBool};

#[derive(Debug, Clone)]
pub(super) struct Choices {
    case_sensitive: bool,
    item_enum: ChoicesItemEnum,
}

impl Choices {
    /// proc_macro_attributeの入力ストリームを基に作成
    ///
    /// - `attr`: 引数
    /// - `item`: 引数でないやつ
    pub(super) fn new(attr: TokenStream, item: TokenStream) -> syn::Result<Self> {
        let case_sensitive = syn::parse::<LitBool>(attr)
            .map_err(|e| syn::Error::new(e.span(), "expected a single bool as an arg"))?
            .value;
        let item_enum = syn::parse::<ItemEnum>(item).map_err(|e| {
            syn::Error::new(e.span(), "expected a single bool as an arg")
        })?;
        let item_enum = ChoicesItemEnum::new(item_enum)?;
        Ok(Self {
            case_sensitive,
            item_enum,
        })
    }

    pub(super) fn get_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
    pub(super) fn get_enum_name(&self) -> proc_macro2::Ident {
        self.item_enum.ident.clone()
    }
    pub(super) fn get_item_enum(&self) -> ChoicesItemEnum {
        self.item_enum.clone()
    }
}

#[derive(Debug, Clone)]
pub(super) struct ChoicesItemEnum(ItemEnum);

impl Deref for ChoicesItemEnum {
    type Target = ItemEnum;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChoicesItemEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ChoicesItemEnum> for ItemEnum {
    fn from(value: ChoicesItemEnum) -> Self {
        value.0
    }
}

impl ChoicesItemEnum {
    pub(super) fn new(item_enum: ItemEnum) -> syn::Result<Self> {
        check_without_fields_enum(&item_enum)?;
        Ok(Self(item_enum))
    }
}

fn check_without_fields_enum(item_enum: &ItemEnum) -> syn::Result<()> {
    if item_enum.variants.iter().any(|v| !matches!(v.fields, syn::Fields::Unit)) {
        Err(syn::Error::new_spanned(
            item_enum.ident.clone(),
            "#[choices] macro can be used with C-style enums (i.e. without fields)",
        ))
    } else {
        Ok(())
    }
}
