//! Mingling Macros Crate
//!
//! This crate provides procedural macros for the Mingling framework.
//! Macros are implemented in separate modules and re-exported here.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

mod chain;
#[cfg(feature = "comp")]
mod completion;
mod dispatcher_chain;
mod groupped;
mod node;
mod pack;
mod program_setup;
mod render;
mod renderer;
#[cfg(feature = "comp")]
mod suggest;

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global variable declarations for storing chain and renderer mappings
#[cfg(feature = "general_renderer")]
pub(crate) static GENERAL_RENDERERS: Lazy<Mutex<Vec<String>>> =
    Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static PACKED_TYPES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
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
pub fn chain(attr: TokenStream, item: TokenStream) -> TokenStream {
    chain::chain_attr(attr, item)
}

#[proc_macro_attribute]
pub fn renderer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    renderer::renderer_attr(item)
}

#[cfg(feature = "comp")]
#[proc_macro_attribute]
pub fn completion(attr: TokenStream, item: TokenStream) -> TokenStream {
    completion::completion_attr(attr, item)
}

#[proc_macro_attribute]
pub fn program_setup(attr: TokenStream, item: TokenStream) -> TokenStream {
    program_setup::setup_attr(attr, item)
}

#[proc_macro_derive(Groupped, attributes(group))]
pub fn derive_groupped(input: TokenStream) -> TokenStream {
    groupped::derive_groupped(input)
}

#[cfg(feature = "general_renderer")]
#[proc_macro_derive(GrouppedSerialize, attributes(group))]
pub fn derive_groupped_serialize(input: TokenStream) -> TokenStream {
    groupped::derive_groupped_serialize(input)
}

#[proc_macro]
pub fn gen_program(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        Ident::new("DefaultProgram", proc_macro2::Span::call_site())
    } else {
        parse_macro_input!(input as Ident)
    };

    let mut packed_types = PACKED_TYPES.lock().unwrap().clone();
    packed_types.push("DispatcherNotFound".to_string());
    packed_types.push("RendererNotFound".to_string());
    packed_types.sort();
    packed_types.dedup();
    let renderers = RENDERERS.lock().unwrap().clone();
    let chains = CHAINS.lock().unwrap().clone();
    let renderer_exist = RENDERERS_EXIST.lock().unwrap().clone();
    let chain_exist = CHAINS_EXIST.lock().unwrap().clone();

    #[cfg(feature = "general_renderer")]
    let general_renderers = GENERAL_RENDERERS.lock().unwrap().clone();

    let packed_types: Vec<proc_macro2::TokenStream> = packed_types
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

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

    #[cfg(feature = "general_renderer")]
    let general_renderer_tokens: Vec<proc_macro2::TokenStream> = general_renderers
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    #[cfg(feature = "general_renderer")]
    let general_render = quote! {
        fn general_render(
            any: ::mingling::AnyOutput<Self::Enum>,
            setting: &::mingling::GeneralRendererSetting,
        ) -> Result<::mingling::RenderResult, ::mingling::error::GeneralRendererSerializeError> {
            match any.member_id {
                #(#general_renderer_tokens)*
                _ => Ok(::mingling::RenderResult::default()),
            }
        }
    };

    #[cfg(not(feature = "general_renderer"))]
    let general_render = quote! {};

    let expanded = quote! {
        ::mingling::macros::pack!(#name, RendererNotFound = String);
        ::mingling::macros::pack!(#name, DispatcherNotFound = Vec<String>);

        #[derive(Debug, Default, PartialEq, Eq, Clone)]
        #[repr(u32)]
        pub enum #name {
            #[default]
            __FallBack,
            #(#packed_types),*
        }

        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #name::__FallBack => write!(f, "__FallBack"),
                    #(#name::#packed_types => write!(f, stringify!(#packed_types)),)*
                }
            }
        }

        impl ::mingling::ProgramCollect for #name {
            type Enum = #name;
            fn build_renderer_not_found(member_id: Self::Enum) -> ::mingling::AnyOutput<Self::Enum> {
                ::mingling::AnyOutput::new(RendererNotFound::new(member_id.to_string()))
            }
            fn build_dispatcher_not_found(args: Vec<String>) -> ::mingling::AnyOutput<Self::Enum> {
                ::mingling::AnyOutput::new(DispatcherNotFound::new(args))
            }
            ::mingling::__dispatch_program_renderers!(
                #(#renderer_tokens)*
            );
            ::mingling::__dispatch_program_chains!(
                #(#chain_tokens)*
            );
            fn has_renderer(any: &::mingling::AnyOutput<Self::Enum>) -> bool {
                match any.member_id {
                    #(#renderer_exist_tokens)*
                    _ => false
                }
            }
            fn has_chain(any: &::mingling::AnyOutput<Self::Enum>) -> bool {
                match any.member_id {
                    #(#chain_exist_tokens)*
                    _ => false
                }
            }
            #general_render
        }

        impl #name {
            pub fn new() -> ::mingling::Program<#name, #name> {
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

#[cfg(feature = "comp")]
#[proc_macro]
pub fn suggest(input: TokenStream) -> TokenStream {
    suggest::suggest(input)
}
