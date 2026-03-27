//! Render Macros Module
//!
//! This module provides procedural macros for rendering operations.
//! These macros expect a mutable reference to a `RenderResult` named `r` to be in scope.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Expr, Token};

/// Implementation of the `r_print!` procedural macro
///
/// This macro expands to a call to `RenderResult::print` with formatted arguments.
/// It expects a mutable reference to a `RenderResult` named `r` to be in scope.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::r_print;
///
/// let mut r = RenderResult::default();
/// r_print!("Hello, {}!", "world");
/// ```
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

/// Implementation of the `r_println!` procedural macro
///
/// This macro expands to a call to `RenderResult::println` with formatted arguments.
/// It expects a mutable reference to a `RenderResult` named `r` to be in scope.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::r_println;
///
/// let mut r = RenderResult::default();
/// r_println!("Hello, {}!", "world");
/// ```
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
