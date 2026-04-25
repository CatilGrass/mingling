//! Dispatcher Chain and Dispatcher Render Macros
//!
//! This module provides macros for creating dispatcher chain and dispatcher render structs
//! with automatic implementations of the `Dispatcher` trait.

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

// NOTICE: This implementation contains significant code duplication between the explicit
// and default cases in both `dispatcher_chain` and `dispatcher_render` functions.
// The logic for handling default vs explicit group names and generating the appropriate
// code should be extracted into common helper functions to reduce redundancy.
// Additionally, the token stream generation patterns are nearly identical between
// the two main functions and could benefit from refactoring.

pub fn dispatcher(input: TokenStream) -> TokenStream {
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

    let comp_entry = get_comp_entry(&pack);

    let expanded = {
        let program_ident = if use_default {
            Ident::new("ThisProgram", proc_macro2::Span::call_site())
        } else {
            group_name.clone()
        };

        quote! {
            #[derive(Debug, Default)]
            pub struct #command_struct;

            ::mingling::macros::pack!(#program_ident, #pack = Vec<String>);

            #comp_entry

            impl ::mingling::Dispatcher<#program_ident> for #command_struct {
                fn node(&self) -> ::mingling::Node {
                    ::mingling::macros::node!(#command_name_str)
                }
                fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess<#program_ident> {
                    #pack::new(args).to_chain()
                }
                fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher<#program_ident>> {
                    Box::new(#command_struct)
                }
            }
        }
    };

    expanded.into()
}

#[cfg(feature = "comp")]
fn get_comp_entry(entry_name: &Ident) -> proc_macro2::TokenStream {
    let comp_entry = quote! {
        impl ::mingling::CompletionEntry for #entry_name {
            fn get_input(self) -> Vec<String> {
                self.inner.clone()
            }
        }
    };
    comp_entry
}

#[cfg(not(feature = "comp"))]
fn get_comp_entry(_entry_name: &Ident) -> proc_macro2::TokenStream {
    quote! {}
}
