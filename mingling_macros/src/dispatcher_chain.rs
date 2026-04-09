//! Dispatcher Chain and Dispatcher Render Macros
//!
//! This module provides macros for creating dispatcher chain and dispatcher render structs
//! with automatic implementations of the `DispatcherChain` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as SynResult, Token};

enum DispatcherChainInput {
    Explicit {
        group_name: Ident,
        command_name: syn::LitStr,
        command_struct: Ident,
        pack: Ident,
    },
    Default {
        command_name: syn::LitStr,
        command_struct: Ident,
        pack: Ident,
    },
}

impl Parse for DispatcherChainInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) && input.peek2(Token![,]) && input.peek3(syn::LitStr) {
            let group_name = input.parse()?;
            input.parse::<Token![,]>()?;
            let command_name = input.parse()?;
            input.parse::<Token![,]>()?;
            let command_struct = input.parse()?;
            input.parse::<Token![=>]>()?;
            let pack = input.parse()?;

            Ok(DispatcherChainInput::Explicit {
                group_name,
                command_name,
                command_struct,
                pack,
            })
        } else if lookahead.peek(syn::LitStr) {
            // Default format: "command_name", CommandStruct => ChainStruct
            let command_name = input.parse()?;
            input.parse::<Token![,]>()?;
            let command_struct = input.parse()?;
            input.parse::<Token![=>]>()?;
            let pack = input.parse()?;

            Ok(DispatcherChainInput::Default {
                command_name,
                command_struct,
                pack,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn dispatcher_chain(input: TokenStream) -> TokenStream {
    // Parse the input
    let dispatcher_input = syn::parse_macro_input!(input as DispatcherChainInput);

    // Determine if we're using default or explicit group
    let (group_name, command_name, command_struct, pack, use_default) = match dispatcher_input {
        DispatcherChainInput::Explicit {
            group_name,
            command_name,
            command_struct,
            pack,
        } => (group_name, command_name, command_struct, pack, false),
        DispatcherChainInput::Default {
            command_name,
            command_struct,
            pack,
        } => (
            Ident::new("ThisProgram", proc_macro2::Span::call_site()),
            command_name,
            command_struct,
            pack,
            true,
        ),
    };

    let command_name_str = command_name.value();

    let expanded = if use_default {
        // For default case, use ThisProgram
        quote! {
            #[derive(Debug, Default)]
            pub struct #command_struct;

            ::mingling::macros::pack!(ThisProgram, #pack = Vec<String>);

            impl ::mingling::Dispatcher<ThisProgram> for #command_struct {
                fn node(&self) -> ::mingling::Node {
                    ::mingling::macros::node!(#command_name_str)
                }
                fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess<ThisProgram> {
                    #pack::new(args).to_chain()
                }
                fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher<ThisProgram>> {
                    Box::new(#command_struct)
                }
            }
        }
    } else {
        // For explicit case, use the provided group_name
        quote! {
            #[derive(Debug, Default)]
            pub struct #command_struct;

            ::mingling::macros::pack!(#group_name, #pack = Vec<String>);

            impl ::mingling::Dispatcher<#group_name> for #command_struct {
                fn node(&self) -> ::mingling::Node {
                    ::mingling::macros::node!(#command_name_str)
                }
                fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess<#group_name> {
                    #pack::new(args).to_chain()
                }
                fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher<#group_name>> {
                    Box::new(#command_struct)
                }
            }
        }
    };

    expanded.into()
}

pub fn dispatcher_render(input: TokenStream) -> TokenStream {
    // Parse the input
    let dispatcher_input = syn::parse_macro_input!(input as DispatcherChainInput);

    // Determine if we're using default or explicit group
    let (group_name, command_name, command_struct, pack, use_default) = match dispatcher_input {
        DispatcherChainInput::Explicit {
            group_name,
            command_name,
            command_struct,
            pack,
        } => (group_name, command_name, command_struct, pack, false),
        DispatcherChainInput::Default {
            command_name,
            command_struct,
            pack,
        } => (
            Ident::new("ThisProgram", proc_macro2::Span::call_site()),
            command_name,
            command_struct,
            pack,
            true,
        ),
    };

    let command_name_str = command_name.value();

    let expanded = if use_default {
        // For default case, use ThisProgram
        quote! {
            #[derive(Debug, Default)]
            pub struct #command_struct;

            ::mingling::macros::pack!(ThisProgram, #pack = Vec<String>);

            impl ::mingling::Dispatcher for #command_struct {
                fn node(&self) -> ::mingling::Node {
                    ::mingling::macros::node!(#command_name_str)
                }
                fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess {
                    #pack::new(args).to_render()
                }
                fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher> {
                    Box::new(#command_struct)
                }
            }
        }
    } else {
        // For explicit case, use the provided group_name
        quote! {
            #[derive(Debug, Default)]
            pub struct #command_struct;

            ::mingling::macros::pack!(#group_name, #pack = Vec<String>);

            impl ::mingling::Dispatcher for #command_struct {
                fn node(&self) -> ::mingling::Node {
                    ::mingling::macros::node!(#command_name_str)
                }
                fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess {
                    #pack::new(args).to_render()
                }
                fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher> {
                    Box::new(#command_struct)
                }
            }
        }
    };

    expanded.into()
}
