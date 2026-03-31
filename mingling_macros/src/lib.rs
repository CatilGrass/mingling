//! Mingling Macros Crate
//!
//! This crate provides procedural macros for the Mingling framework.
//! Macros are implemented in separate modules and re-exported here.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

mod chain;
mod dispatcher_chain;
mod node;
mod pack;
mod render;
mod renderer;

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global variable declarations for storing chain and renderer mappings
pub(crate) static CHAINS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static RENDERERS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static CHAINS_EXIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static RENDERERS_EXIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[proc_macro]
pub fn node(input: TokenStream) -> TokenStream {
    node::node(input)
}

#[proc_macro]
pub fn pack(input: TokenStream) -> TokenStream {
    pack::pack(input)
}

#[proc_macro]
pub fn dispatcher(input: TokenStream) -> TokenStream {
    dispatcher_chain::dispatcher_chain(input)
}

#[proc_macro]
pub fn dispatcher_render(input: TokenStream) -> TokenStream {
    dispatcher_chain::dispatcher_render(input)
}

#[proc_macro]
pub fn r_print(input: TokenStream) -> TokenStream {
    render::r_print(input)
}

#[proc_macro]
pub fn r_println(input: TokenStream) -> TokenStream {
    render::r_println(input)
}

#[proc_macro_attribute]
pub fn chain(_attr: TokenStream, item: TokenStream) -> TokenStream {
    chain::chain_attr(item)
}

#[proc_macro_attribute]
pub fn renderer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    renderer::renderer_attr(item)
}

#[proc_macro]
pub fn program(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let renderers = RENDERERS.lock().unwrap().clone();
    let chains = CHAINS.lock().unwrap().clone();
    let renderer_exist = RENDERERS_EXIST.lock().unwrap().clone();
    let chain_exist = CHAINS_EXIST.lock().unwrap().clone();

    let renderer_tokens: Vec<proc_macro2::TokenStream> = renderers
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let chain_tokens: Vec<proc_macro2::TokenStream> = chains
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let renderer_exist_tokens: Vec<proc_macro2::TokenStream> = renderer_exist
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let chain_exist_tokens: Vec<proc_macro2::TokenStream> = chain_exist
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let expanded = quote! {
        pub struct #name;

        impl ::mingling::ProgramCollect for #name {
            ::mingling::__dispatch_program_renderers!(
                #(#renderer_tokens)*
            );
            ::mingling::__dispatch_program_chains!(
                #(#chain_tokens)*
            );
            fn has_renderer(any: &::mingling::AnyOutput) -> bool {
                match any.type_id {
                    #(#renderer_exist_tokens)*
                    _ => false
                }
            }
            fn has_chain(any: &::mingling::AnyOutput) -> bool {
                match any.type_id {
                    #(#chain_exist_tokens)*
                    _ => false
                }
            }
        }

        impl #name {
            pub fn new() -> ::mingling::Program<#name> {
                ::mingling::Program::new()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Internal macro for registering chains.
///
/// This macro is used internally by the `#[chain]` attribute macro
/// and should not be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __register_chain(input: TokenStream) -> TokenStream {
    let chain_entry = parse_macro_input!(input as syn::LitStr);
    let entry_str = chain_entry.value();

    CHAINS.lock().unwrap().push(entry_str);

    TokenStream::new()
}

/// Internal macro for registering renderers.
///
/// This macro is used internally by the `#[renderer]` attribute macro
/// and should not be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __register_renderer(input: TokenStream) -> TokenStream {
    let renderer_entry = parse_macro_input!(input as syn::LitStr);
    let entry_str = renderer_entry.value();

    RENDERERS.lock().unwrap().push(entry_str);

    TokenStream::new()
}
