//! Setup Attribute Macro Implementation
//!
//! This module provides the `#[setup]` attribute macro for automatically
//! generating structs that implement the `ProgramSetup` trait from functions.

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Signature, Type, TypePath, parse_macro_input,
};

/// Extracts the program parameter from function arguments
fn extract_program_param(sig: &Signature) -> syn::Result<(Pat, TypePath)> {
    // The function should have exactly one parameter
    if sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "Setup function must have exactly one parameter",
        ));
    }

    let arg = &sig.inputs[0];
    match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => {
            // Extract the pattern (parameter name)
            let param_pat = (**pat).clone();

            // Extract the type, handling references like &mut ThisProgram
            match &**ty {
                Type::Path(type_path) => Ok((param_pat, type_path.clone())),
                Type::Reference(type_ref) => {
                    // Handle reference types like &mut ThisProgram
                    match &*type_ref.elem {
                        Type::Path(type_path) => Ok((param_pat, type_path.clone())),
                        _ => Err(syn::Error::new(
                            ty.span(),
                            "Reference parameter must point to a type path",
                        )),
                    }
                }
                _ => Err(syn::Error::new(
                    ty.span(),
                    "Parameter type must be a type path or reference to a type path",
                )),
            }
        }
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "Setup function cannot have self parameter",
        )),
    }
}

/// Validates that the parameter type is `ThisProgram`
fn validate_any_program_param(type_path: &TypePath) -> syn::Result<()> {
    // Check if the type is `ThisProgram`
    let segments = &type_path.path.segments;
    if segments.len() == 1 && segments[0].ident == "ThisProgram" {
        Ok(())
    } else {
        // Check if it's a qualified path like mingling::marker::ThisProgram
        let mut is_any_program = false;
        if segments.len() > 1 {
            // Check if the last segment is "ThisProgram"
            if segments.last().unwrap().ident == "ThisProgram" {
                is_any_program = true;
            }
        }

        if is_any_program {
            Ok(())
        } else {
            Err(syn::Error::new(
                type_path.span(),
                "Setup function parameter must be `mingling::marker::ThisProgram`",
            ))
        }
    }
}

/// Extracts and validates the return type
fn extract_return_type(sig: &Signature) -> syn::Result<()> {
    // Setup functions should return () or have no return type
    match &sig.output {
        ReturnType::Type(_, ty) => {
            // Check if it's ()
            match &**ty {
                Type::Tuple(tuple) if tuple.elems.is_empty() => Ok(()),
                _ => Err(syn::Error::new(
                    ty.span(),
                    "Setup function must return () or have no return type",
                )),
            }
        }
        ReturnType::Default => Ok(()),
    }
}

pub fn setup_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments (e.g., MyProgram from #[setup(MyProgram)])
    // If no argument is provided, use DefaultProgram
    let (program_name, use_crate_prefix) = if attr.is_empty() {
        (
            Ident::new("DefaultProgram", proc_macro2::Span::call_site()),
            true,
        )
    } else {
        (parse_macro_input!(attr as Ident), false)
    };

    // Parse the function item
    let input_fn = parse_macro_input!(item as ItemFn);

    // Validate the function is not async
    if input_fn.sig.asyncness.is_some() {
        return syn::Error::new(input_fn.sig.span(), "Setup function cannot be async")
            .to_compile_error()
            .into();
    }

    // Extract the program parameter
    let (program_param, program_type) = match extract_program_param(&input_fn.sig) {
        Ok(info) => info,
        Err(e) => return e.to_compile_error().into(),
    };

    // Validate that the parameter is ThisProgram
    if let Err(e) = validate_any_program_param(&program_type) {
        return e.to_compile_error().into();
    }

    // Validate return type
    if let Err(e) = extract_return_type(&input_fn.sig) {
        return e.to_compile_error().into();
    }

    // Get the function body
    let fn_body = &input_fn.block;

    // Get function attributes (excluding the setup attribute)
    let mut fn_attrs = input_fn.attrs.clone();

    // Remove any #[setup(...)] attributes to avoid infinite recursion
    fn_attrs.retain(|attr| !attr.path().is_ident("setup"));

    // Get function visibility
    let vis = &input_fn.vis;

    // Get function name
    let fn_name = &input_fn.sig.ident;

    // Generate struct name from function name using pascal_case
    let pascal_case_name = just_fmt::pascal_case!(fn_name.to_string());
    let struct_name = Ident::new(&pascal_case_name, fn_name.span());

    // Generate the struct and implementation
    let expanded = if use_crate_prefix {
        quote! {
            #(#fn_attrs)*
            #vis struct #struct_name;

            impl ::mingling::setup::ProgramSetup<DefaultProgram, DefaultProgram> for #struct_name {
                fn setup(&mut self, program: &mut ::mingling::Program<DefaultProgram, DefaultProgram>) {
                    let _ = ThisProgram;
                    // Call the original function with the actual Program type
                    #fn_name(program);
                }
            }

            // Keep the original function for internal use
            #(#fn_attrs)*
            #vis fn #fn_name(#program_param: &mut ::mingling::Program<DefaultProgram, DefaultProgram>) {
                #fn_body
            }
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #vis struct #struct_name;

            impl ::mingling::setup::ProgramSetup<#program_name, #program_name> for #struct_name {
                fn setup(&mut self, program: &mut ::mingling::Program<#program_name, #program_name>) {
                    let _ = ThisProgram;
                    // Call the original function with the actual Program type
                    #fn_name(program);
                }
            }

            // Keep the original function for internal use
            #(#fn_attrs)*
            #vis fn #fn_name(#program_param: &mut ::mingling::Program<#program_name, #program_name>) {
                #fn_body
            }
        }
    };

    expanded.into()
}
