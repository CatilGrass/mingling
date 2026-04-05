//! Groupped Derive Macro Implementation
//!
//! This module provides the `#[derive(Groupped)]` macro for automatically
//! implementing the `Groupped` trait on structs and enums.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, parse_macro_input};

/// Parses the `#[group(...)]` attribute to extract the group type
fn parse_group_attribute(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if attr.path().is_ident("group") {
            if let Ok(meta) = attr.parse_args::<syn::Meta>() {
                if let syn::Meta::Path(path) = meta {
                    if let Some(segment) = path.segments.last() {
                        return Some(segment.ident.clone());
                    }
                }
            }
        }
    }
    None
}

pub fn derive_groupped(input: TokenStream) -> TokenStream {
    // Parse the input struct/enum
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Parse attributes to find #[group(...)]
    let group_ident = parse_group_attribute(&input.attrs)
        .unwrap_or_else(|| Ident::new("DefaultProgram", Span::call_site()));

    // Generate the Groupped trait implementation
    let expanded = quote! {
        impl ::mingling::Groupped<#group_ident> for #struct_name {
            fn member_id() -> #group_ident {
                #group_ident::#struct_name
            }
        }
    };

    expanded.into()
}

#[cfg(feature = "general_renderer")]
pub fn derive_groupped_serialize(input: TokenStream) -> TokenStream {
    // Parse the input struct/enum
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let struct_name = input_parsed.ident.clone();

    // Parse attributes to find #[group(...)]
    let group_ident = parse_group_attribute(&input_parsed.attrs)
        .unwrap_or_else(|| Ident::new("DefaultProgram", Span::call_site()));

    // Generate both Serialize and Groupped implementations
    let expanded = quote! {
        #[derive(serde::Serialize)]
        #input_parsed

        impl ::mingling::Groupped<#group_ident> for #struct_name {
            fn member_id() -> #group_ident {
                #group_ident::#struct_name
            }
        }
    };

    expanded.into()
}
