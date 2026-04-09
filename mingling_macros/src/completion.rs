//! Completion Attribute Macro Implementation
//!
//! This module provides the `#[completion]` attribute macro for automatically
//! generating structs that implement the `Completion` trait from functions.

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Signature, Type, TypePath, parse_macro_input,
};

/// Extracts the previous type from function arguments
fn extract_previous_type(sig: &Signature) -> syn::Result<TypePath> {
    // The function should have exactly one parameter: ShellContext
    if sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "Completion function must have exactly one parameter (ShellContext)",
        ));
    }

    let arg = &sig.inputs[0];
    match arg {
        FnArg::Typed(PatType { ty, .. }) => {
            match &**ty {
                Type::Path(type_path) => {
                    // Check if it's ShellContext
                    let last_segment = type_path.path.segments.last().unwrap();
                    if last_segment.ident != "ShellContext" {
                        return Err(syn::Error::new(
                            ty.span(),
                            "Parameter type must be ShellContext",
                        ));
                    }
                    Ok(type_path.clone())
                }
                _ => Err(syn::Error::new(
                    ty.span(),
                    "Parameter type must be a type path",
                )),
            }
        }
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "Completion function cannot have self parameter",
        )),
    }
}

/// Extracts the return type from the function signature
fn extract_return_type(sig: &Signature) -> syn::Result<TypePath> {
    match &sig.output {
        ReturnType::Type(_, ty) => match &**ty {
            Type::Path(type_path) => {
                // Check if it's Suggest
                let last_segment = type_path.path.segments.last().unwrap();
                if last_segment.ident != "Suggest" {
                    return Err(syn::Error::new(ty.span(), "Return type must be Suggest"));
                }
                Ok(type_path.clone())
            }
            _ => Err(syn::Error::new(
                ty.span(),
                "Return type must be a type path",
            )),
        },
        ReturnType::Default => Err(syn::Error::new(
            sig.span(),
            "Completion function must have a return type",
        )),
    }
}

/// Extracts the parameter name from function arguments
fn extract_param_name(sig: &Signature) -> syn::Result<Pat> {
    if sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "Completion function must have exactly one parameter",
        ));
    }

    let arg = &sig.inputs[0];
    match arg {
        FnArg::Typed(PatType { pat, .. }) => Ok((**pat).clone()),
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "Completion function cannot have self parameter",
        )),
    }
}

#[cfg(feature = "comp")]
pub fn completion_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments (e.g., HelloEntry from #[completion(HelloEntry)])
    let previous_type_ident = if attr.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "completion attribute requires a previous type argument, e.g. #[completion(HelloEntry)]",
        )
        .to_compile_error()
        .into();
    } else {
        parse_macro_input!(attr as Ident)
    };

    // Parse the function item
    let input_fn = parse_macro_input!(item as ItemFn);

    // Validate the function is not async
    if input_fn.sig.asyncness.is_some() {
        return syn::Error::new(input_fn.sig.span(), "Completion function cannot be async")
            .to_compile_error()
            .into();
    }

    // Extract the parameter name
    let param_name = match extract_param_name(&input_fn.sig) {
        Ok(name) => name,
        Err(e) => return e.to_compile_error().into(),
    };

    // Extract and validate the parameter type (must be ShellContext)
    if let Err(e) = extract_previous_type(&input_fn.sig) {
        return e.to_compile_error().into();
    }

    // Extract and validate the return type (must be Suggest)
    if let Err(e) = extract_return_type(&input_fn.sig) {
        return e.to_compile_error().into();
    }

    // Get the function body
    let fn_body = &input_fn.block;

    // Get function attributes (excluding the completion attribute)
    let mut fn_attrs = input_fn.attrs.clone();
    fn_attrs.retain(|attr| !attr.path().is_ident("completion"));

    // Get function visibility
    let vis = &input_fn.vis;

    // Get function name
    let fn_name = &input_fn.sig.ident;

    // Generate struct name from function name using pascal_case
    let pascal_case_name = just_fmt::pascal_case!(fn_name.to_string());
    let struct_name = Ident::new(&pascal_case_name, fn_name.span());

    // Generate the struct and implementation
    let expanded = quote! {
        #(#fn_attrs)*
        #vis struct #struct_name;

        impl ::mingling::Completion for #struct_name {
            type Previous = #previous_type_ident;

            fn comp(#param_name: ::mingling::ShellContext) -> ::mingling::Suggest {
                // This is just to prevent warnings about imported ShellContext and Suggest
                let _ = ShellContext::default();
                let _ = Suggest::file_comp();
                #fn_body
            }
        }

        // Keep the original function for internal use
        #(#fn_attrs)*
        #vis fn #fn_name(#param_name: ::mingling::ShellContext) -> ::mingling::Suggest {
            #fn_body
        }
    };

    expanded.into()
}
