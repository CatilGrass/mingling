//! Dispatcher Chain and Dispatcher Render Macros
//!
//! This module provides macros for creating dispatcher chain and dispatcher render structs
//! with automatic implementations of the `DispatcherChain` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as SynResult, Token};

/// Parses input in the format: `"command_name", CommandStruct => ChainStruct`
struct DispatcherChainInput {
    command_name: syn::LitStr,
    command_struct: Ident,
    chain_struct: Ident,
}

impl Parse for DispatcherChainInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let command_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let command_struct = input.parse()?;
        input.parse::<Token![=>]>()?;
        let chain_struct = input.parse()?;

        Ok(DispatcherChainInput {
            command_name,
            command_struct,
            chain_struct,
        })
    }
}

pub fn dispatcher_chain(input: TokenStream) -> TokenStream {
    let DispatcherChainInput {
        command_name,
        command_struct,
        chain_struct,
    } = syn::parse_macro_input!(input as DispatcherChainInput);

    let command_name_str = command_name.value();

    let expanded = quote! {
        #[derive(Debug, Default)]
        pub struct #command_struct;

        ::mingling::macros::chain_struct!(#chain_struct = Vec<String>);

        impl ::mingling::Dispatcher for #command_struct {
            fn node(&self) -> ::mingling::Node {
                ::mingling::macros::node!(#command_name_str)
            }
            fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess {
                #chain_struct::new(args).to_chain()
            }
            fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher> {
                Box::new(#command_struct)
            }
        }
    };

    expanded.into()
}

pub fn dispatcher_render(input: TokenStream) -> TokenStream {
    let DispatcherChainInput {
        command_name,
        command_struct,
        chain_struct,
    } = syn::parse_macro_input!(input as DispatcherChainInput);

    let command_name_str = command_name.value();

    let expanded = quote! {
        #[derive(Debug, Default)]
        pub struct #command_struct;

        ::mingling::macros::chain_struct!(#chain_struct = Vec<String>);

        impl ::mingling::Dispatcher for #command_struct {
            fn node(&self) -> ::mingling::Node {
                ::mingling::macros::node!(#command_name_str)
            }
            fn begin(&self, args: Vec<String>) -> ::mingling::ChainProcess {
                #chain_struct::new(args).to_render()
            }
            fn clone_dispatcher(&self) -> Box<dyn ::mingling::Dispatcher> {
                Box::new(#command_struct)
            }
        }
    };

    expanded.into()
}
