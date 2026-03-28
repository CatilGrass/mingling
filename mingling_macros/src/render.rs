//! Render Macros Module
//!
//! This module provides procedural macros for rendering operations.
//! These macros expect a mutable reference to a `RenderResult` named `r` to be in scope.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Expr, Token};

pub fn r_print(input: TokenStream) -> TokenStream {
    // Parse the input as format arguments
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let format_args = match parser.parse(input) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    // Build the format macro call
    let format_call = if format_args.is_empty() {
        quote! { ::std::format!("") }
    } else {
        let args_iter = format_args.iter();
        quote! { ::std::format!(#(#args_iter),*) }
    };

    let expanded = quote! {
        {
            let formatted = #format_call;
            ::mingling::RenderResult::print(r, &formatted)
        }
    };

    expanded.into()
}

pub fn r_println(input: TokenStream) -> TokenStream {
    // Parse the input as format arguments
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let format_args = match parser.parse(input) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    // Build the format macro call
    let format_call = if format_args.is_empty() {
        quote! { ::std::format!("") }
    } else {
        let args_iter = format_args.iter();
        quote! { ::std::format!(#(#args_iter),*) }
    };

    let expanded = quote! {
        {
            let formatted = #format_call;
            ::mingling::RenderResult::println(r, &formatted)
        }
    };

    expanded.into()
}
