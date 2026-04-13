//! Mingling Macros Crate
//!
//! This crate provides procedural macros for the Mingling framework.
//! Macros are implemented in separate modules and re-exported here.

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::collections::BTreeSet;
use std::sync::Mutex;
use syn::parse_macro_input;

mod chain;
#[cfg(feature = "comp")]
mod completion;
mod dispatcher_chain;
mod enum_tag;
mod groupped;
mod node;
mod pack;
mod program_setup;
mod render;
mod renderer;
#[cfg(feature = "comp")]
mod suggest;

// Global variables
#[cfg(feature = "general_renderer")]
pub(crate) static GENERAL_RENDERERS: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));
#[cfg(feature = "comp")]
pub(crate) static COMPLETIONS: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));

pub(crate) static PACKED_TYPES: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));
pub(crate) static CHAINS: Lazy<Mutex<BTreeSet<String>>> = Lazy::new(|| Mutex::new(BTreeSet::new()));
pub(crate) static RENDERERS: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));
pub(crate) static CHAINS_EXIST: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));
pub(crate) static RENDERERS_EXIST: Lazy<Mutex<BTreeSet<String>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));

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

#[proc_macro_derive(EnumTag, attributes(enum_desc, enum_rename))]
pub fn derive_enum_tag(input: TokenStream) -> TokenStream {
    enum_tag::derive_enum_tag(input)
}

#[cfg(feature = "general_renderer")]
#[proc_macro_derive(GrouppedSerialize, attributes(group))]
pub fn derive_groupped_serialize(input: TokenStream) -> TokenStream {
    groupped::derive_groupped_serialize(input)
}

#[proc_macro]
pub fn gen_program(input: TokenStream) -> TokenStream {
    let name = read_name(&input);

    #[cfg(feature = "comp")]
    let out = TokenStream::from(quote! {
        ::mingling::macros::program_gen_completion!(#name);
        ::mingling::macros::program_final_gen!(#name);
    });
    #[cfg(not(feature = "comp"))]
    let out = TokenStream::from(quote! {
        ::mingling::macros::program_final_gen!(#name);
    });

    out
}

#[proc_macro]
#[cfg(feature = "comp")]
pub fn program_gen_completion(input: TokenStream) -> TokenStream {
    let name = read_name(&input);

    let comp_dispatcher = quote! {
        #[allow(unused)]
        use __completion_gen::*;
        pub mod __completion_gen {
            use super::*;
            use mingling::marker::NextProcess;
            ::mingling::macros::dispatcher!(#name, "__comp", CompletionDispatcher => CompletionContext);
            ::mingling::macros::pack!(
                #name,
                CompletionSuggest = (::mingling::ShellContext, ::mingling::Suggest)
            );

            #[::mingling::macros::chain(#name)]
            pub async fn __exec_completion(prev: CompletionContext) -> NextProcess {
                let read_ctx = ::mingling::ShellContext::try_from(prev.inner);
                match read_ctx {
                    Ok(ctx) => {
                        let suggest = ::mingling::CompletionHelper::exec_completion::<#name>(&ctx);
                        CompletionSuggest::new((ctx, suggest)).to_render()
                    }
                    Err(_) => std::process::exit(1),
                }
            }

            #[::mingling::macros::renderer(#name)]
            pub fn __render_completion(prev: CompletionSuggest) {
                let (ctx, suggest) = prev.inner;
                ::mingling::CompletionHelper::render_suggest::<#name>(ctx, suggest);
            }
        }
    };

    TokenStream::from(comp_dispatcher)
}

#[proc_macro]
pub fn program_final_gen(input: TokenStream) -> TokenStream {
    let name = read_name(&input);

    let mut packed_types = PACKED_TYPES.lock().unwrap().clone();
    packed_types.insert("DispatcherNotFound".to_string());
    packed_types.insert("RendererNotFound".to_string());

    let renderers = RENDERERS.lock().unwrap().clone();
    let chains = CHAINS.lock().unwrap().clone();
    let renderer_exist = RENDERERS_EXIST.lock().unwrap().clone();
    let chain_exist = CHAINS_EXIST.lock().unwrap().clone();

    #[cfg(feature = "general_renderer")]
    let general_renderers = GENERAL_RENDERERS.lock().unwrap().clone();

    #[cfg(feature = "comp")]
    let completions = COMPLETIONS.lock().unwrap().clone();

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

    #[cfg(feature = "comp")]
    let completion_tokens: Vec<proc_macro2::TokenStream> = completions
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    #[cfg(feature = "comp")]
    let comp = quote! {
        fn do_comp(any: &::mingling::AnyOutput<Self::Enum>, ctx: &::mingling::ShellContext) -> ::mingling::Suggest {
            match any.member_id {
                #(#completion_tokens)*
                _ => ::mingling::Suggest::FileCompletion,
            }
        }
    };

    #[cfg(not(feature = "comp"))]
    let comp = quote! {};

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
            #comp
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

    CHAINS.lock().unwrap().insert(entry_str);

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

    RENDERERS.lock().unwrap().insert(entry_str);

    TokenStream::new()
}

#[cfg(feature = "comp")]
#[proc_macro]
pub fn suggest(input: TokenStream) -> TokenStream {
    suggest::suggest(input)
}

#[cfg(feature = "comp")]
#[proc_macro]
pub fn suggest_enum(input: TokenStream) -> TokenStream {
    suggest::suggest_enum(input)
}

fn read_name(input: &TokenStream) -> Ident {
    if input.is_empty() {
        Ident::new("ThisProgram", proc_macro2::Span::call_site())
    } else {
        syn::parse(input.clone()).unwrap()
    }
}
