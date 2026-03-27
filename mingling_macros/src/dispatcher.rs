//! Dispatcher Derive Macro Implementation
//!
//! This module provides the `Dispatcher` derive macro for automatically
//! implementing the `mingling::Dispatcher` trait for structs.

use just_fmt::dot_case;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue, parse_macro_input};

/// Parses the `#[dispatcher("path")]` attribute if present
fn parse_dispatcher_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("dispatcher") {
            match attr.parse_args::<Meta>() {
                Ok(Meta::NameValue(MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }),
                    ..
                })) => {
                    return Some(lit_str.value());
                }
                Ok(_) => {
                    // If it's not a string literal, we'll use a default
                    return None;
                }
                Err(_) => {
                    // If parsing fails, we'll use a default
                    return None;
                }
            }
        }
    }
    None
}

/// Generates the command node path from the struct name or attribute
fn generate_command_path(struct_name: &Ident, attr_path: Option<String>) -> String {
    if let Some(path) = attr_path {
        path
    } else {
        // Convert struct name to dot_case for default path using the dot_case! macro
        dot_case!(struct_name.to_string())
    }
}

/// Implementation of the `Dispatcher` derive macro
pub fn dispatcher_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    // Parse the dispatcher attribute if present
    let attr_path = parse_dispatcher_attribute(&input.attrs);

    // Generate the command path
    let command_path = generate_command_path(struct_name, attr_path);

    // Generate the implementation
    let expanded = quote! {
        impl ::mingling::Dispatcher for #struct_name {
            fn node(&self) -> ::mingling::Node {
                ::mingling::macros::node!(#command_path)
            }
        }
    };

    expanded.into()
}
