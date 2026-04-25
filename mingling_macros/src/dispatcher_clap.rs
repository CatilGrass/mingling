//! Dispatcher Clap Attribute Macro
//!
//! This module provides the `#[dispatcher_clap(...)]` attribute macro for
//! automatically generating a `Dispatcher` implementation that uses `clap::Parser`
//! to parse command arguments into the annotated struct.
//!
//! This macro is only available when the `clap_parser` feature is enabled.
//!
//! # Syntax
//!
//! ## Without error type (parse failure calls `e.exit()`):
//!
//! ```rust,ignore
//! #[derive(Groupped, clap::Parser)]
//! #[dispatcher_clap("command_name", DispatcherName)]
//! struct MyEntry {
//!     #[arg(long, short)]
//!     name: String,
//! }
//! ```
//!
//! ## With error type (parse failure routes to error struct):
//!
//! ```rust,ignore
//! #[derive(Groupped, clap::Parser)]
//! #[dispatcher_clap("command_name", DispatcherName, error = ParseError)]
//! struct MyEntry {
//!     #[arg(long, short)]
//!     name: String,
//! }
//! ```
//!
//! When `error = ErrorType` is specified, a pack type named `ErrorType` is generated
//! that wraps the clap error message as a `String`. On parse failure, the error
//! message is routed to the renderer via `to_render()` instead of calling `e.exit()`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemStruct, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Input for the dispatcher_clap attribute
///
/// Two forms:
/// - `("command_name", DispatcherStruct)`
/// - `("command_name", DispatcherStruct, error = ErrorStruct)`
enum DispatcherClapInput {
    /// No error type: `("cmd", DispatcherStruct)`
    Simple {
        command_name: LitStr,
        dispatcher_struct: Ident,
    },
    /// With error type: `("cmd", DispatcherStruct, error = ErrorStruct)`
    WithError {
        command_name: LitStr,
        dispatcher_struct: Ident,
        error_struct: Ident,
    },
}

impl Parse for DispatcherClapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let command_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let dispatcher_struct: Ident = input.parse()?;

        // Check if there's `, error = ErrorStruct`
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let error_ident: Ident = input.parse()?;
            if error_ident != "error" {
                return Err(syn::Error::new(
                    error_ident.span(),
                    "expected `error` keyword",
                ));
            }
            input.parse::<Token![=]>()?;
            let error_struct: Ident = input.parse()?;
            Ok(DispatcherClapInput::WithError {
                command_name,
                dispatcher_struct,
                error_struct,
            })
        } else {
            Ok(DispatcherClapInput::Simple {
                command_name,
                dispatcher_struct,
            })
        }
    }
}

pub fn dispatcher_clap_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let attr_input = parse_macro_input!(attr as DispatcherClapInput);

    // Parse the struct item to get the struct name
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident;

    let expanded = match attr_input {
        DispatcherClapInput::Simple {
            command_name,
            dispatcher_struct,
        } => {
            let command_name_str = command_name.value();
            quote! {
                // Keep the original struct definition
                #input_struct

                // Generate the dispatcher struct
                #[doc(hidden)]
                pub struct #dispatcher_struct;

                impl ::mingling::Dispatcher<ThisProgram> for #dispatcher_struct {
                    fn node(&self) -> ::mingling::Node {
                        ::mingling::macros::node!(#command_name_str)
                    }

                    fn begin(
                        &self,
                        args: Vec<String>,
                    ) -> ::mingling::ChainProcess<ThisProgram> {
                        // Prepend a dummy program name for clap's parse_from
                        let clap_args = std::iter::once(String::new())
                            .chain(args)
                            .collect::<Vec<_>>();

                        // Parse using clap's Parser, exit on error
                        let parsed = <#struct_name as ::clap::Parser>::try_parse_from(clap_args)
                            .unwrap_or_else(|e| e.exit());

                        parsed.to_chain()
                    }

                    fn clone_dispatcher(
                        &self,
                    ) -> Box<dyn ::mingling::Dispatcher<ThisProgram>> {
                        Box::new(#dispatcher_struct)
                    }
                }
            }
        }
        DispatcherClapInput::WithError {
            command_name,
            dispatcher_struct,
            error_struct,
        } => {
            let command_name_str = command_name.value();
            quote! {
                // Keep the original struct definition
                #input_struct

                // Generate the error wrapper type via pack!
                ::mingling::macros::pack!(#error_struct = String);

                // Generate the dispatcher struct
                #[doc(hidden)]
                pub struct #dispatcher_struct;

                impl ::mingling::Dispatcher<ThisProgram> for #dispatcher_struct {
                    fn node(&self) -> ::mingling::Node {
                        ::mingling::macros::node!(#command_name_str)
                    }

                    fn begin(
                        &self,
                        args: Vec<String>,
                    ) -> ::mingling::ChainProcess<ThisProgram> {
                        // Prepend a dummy program name for clap's parse_from
                        let clap_args = std::iter::once(String::new())
                            .chain(args)
                            .collect::<Vec<_>>();

                        // Parse using clap's Parser, route error on failure
                        match <#struct_name as ::clap::Parser>::try_parse_from(clap_args) {
                            Ok(parsed) => parsed.to_chain(),
                            Err(e) => #error_struct::new(e.to_string()).to_render(),
                        }
                    }

                    fn clone_dispatcher(
                        &self,
                    ) -> Box<dyn ::mingling::Dispatcher<ThisProgram>> {
                        Box::new(#dispatcher_struct)
                    }
                }
            }
        }
    };

    expanded.into()
}
