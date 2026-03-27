//! Renderer Attribute Macro Implementation
//!
//! This module provides the `#[renderer]` attribute macro for automatically
//! generating structs that implement the `Renderer` trait from functions.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Signature, Type, TypePath, parse_macro_input,
};

/// Parses the renderer attribute arguments
struct RendererAttribute {
    struct_name: Ident,
}

impl Parse for RendererAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name = input.parse()?;
        Ok(RendererAttribute { struct_name })
    }
}

/// Extracts the previous type and parameter name from function arguments
fn extract_previous_info(sig: &Signature) -> syn::Result<(Pat, TypePath)> {
    // The function should have exactly two parameters
    if sig.inputs.len() != 2 {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "Renderer function must have exactly two parameters",
        ));
    }

    // First parameter is the previous type
    let arg = &sig.inputs[0];
    match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => {
            // Extract the pattern (parameter name)
            let param_pat = (**pat).clone();

            // Extract the type
            match &**ty {
                Type::Path(type_path) => Ok((param_pat, type_path.clone())),
                _ => Err(syn::Error::new(
                    ty.span(),
                    "First parameter type must be a type path",
                )),
            }
        }
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "Renderer function cannot have self parameter",
        )),
    }
}

/// Validates that the second parameter is r: &mut RenderResult
fn validate_render_result_param(sig: &Signature) -> syn::Result<()> {
    // Second parameter should be &mut RenderResult
    let arg = &sig.inputs[1];

    match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => {
            // Check parameter name is "r"
            let param_name = match &**pat {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => {
                    return Err(syn::Error::new(
                        pat.span(),
                        "Second parameter must be named 'r'",
                    ));
                }
            };

            if param_name != "r" {
                return Err(syn::Error::new(
                    pat.span(),
                    "Second parameter must be named 'r'",
                ));
            }

            // Check type is &mut RenderResult
            match &**ty {
                Type::Reference(type_ref) => {
                    // Check mutability
                    if !type_ref.mutability.is_some() {
                        return Err(syn::Error::new(
                            ty.span(),
                            "Second parameter must be mutable reference: &mut RenderResult",
                        ));
                    }

                    // Check inner type is RenderResult
                    match &*type_ref.elem {
                        Type::Path(type_path) => {
                            let type_name =
                                type_path.path.segments.last().unwrap().ident.to_string();
                            if type_name != "RenderResult" {
                                return Err(syn::Error::new(
                                    ty.span(),
                                    "Second parameter must be &mut RenderResult",
                                ));
                            }
                        }
                        _ => {
                            return Err(syn::Error::new(
                                ty.span(),
                                "Second parameter must be &mut RenderResult",
                            ));
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ty.span(),
                        "Second parameter must be &mut RenderResult",
                    ));
                }
            }
        }
        FnArg::Receiver(_) => {
            return Err(syn::Error::new(
                arg.span(),
                "Renderer function cannot have self parameter",
            ));
        }
    }

    Ok(())
}

/// Extracts the return type from the function signature
fn extract_return_type(sig: &Signature) -> syn::Result<()> {
    // Renderer functions should return () or have no return type
    match &sig.output {
        ReturnType::Type(_, ty) => {
            // Check if it's ()
            match &**ty {
                Type::Tuple(tuple) if tuple.elems.is_empty() => Ok(()),
                _ => Err(syn::Error::new(
                    ty.span(),
                    "Renderer function must return () or have no return type",
                )),
            }
        }
        ReturnType::Default => Ok(()),
    }
}

/// Implementation of the `#[renderer]` attribute macro
///
/// This macro transforms a function into a struct that implements
/// the `Renderer` trait. The struct name is specified in the attribute.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::renderer;
///
/// #[renderer(InitResultRenderer)]
/// fn render(data: InitResult, r: &mut RenderResult) {
///     let str: String = data.into();
///     r_println!("{}", str);
/// }
/// ```
///
/// This generates:
/// ```ignore
/// pub struct InitResultRenderer;
/// impl Renderer for InitResultRenderer {
///     type Previous = InitResult;
///
///     fn render(data: Self::Previous, r: &mut RenderResult) {
///         let str: String = data.into();
///         r_println!("{}", str);
///     }
/// }
/// ```
pub fn renderer_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let renderer_attr = parse_macro_input!(attr as RendererAttribute);
    let struct_name = renderer_attr.struct_name;

    // Parse the function item
    let input_fn = parse_macro_input!(item as ItemFn);

    // Validate the function is not async
    if input_fn.sig.asyncness.is_some() {
        return syn::Error::new(input_fn.sig.span(), "Renderer function cannot be async")
            .to_compile_error()
            .into();
    }

    // Extract the previous type and parameter name from function arguments
    let (prev_param, previous_type) = match extract_previous_info(&input_fn.sig) {
        Ok(info) => info,
        Err(e) => return e.to_compile_error().into(),
    };

    // Validate second parameter is r: &mut RenderResult
    if let Err(e) = validate_render_result_param(&input_fn.sig) {
        return e.to_compile_error().into();
    }

    // Validate return type
    if let Err(e) = extract_return_type(&input_fn.sig) {
        return e.to_compile_error().into();
    }

    // Get the function body
    let fn_body = &input_fn.block;

    // Get function attributes (excluding the renderer attribute)
    let mut fn_attrs = input_fn.attrs.clone();
    // Remove any #[renderer(...)] attributes to avoid infinite recursion
    fn_attrs.retain(|attr| !attr.path().is_ident("renderer"));

    // Get function visibility
    let vis = &input_fn.vis;

    // Get function name
    let fn_name = &input_fn.sig.ident;

    // Generate the struct and implementation
    let expanded = quote! {
        #(#fn_attrs)*
        #vis struct #struct_name;

        impl ::mingling::Renderer for #struct_name {
            type Previous = #previous_type;

            fn render(#prev_param: Self::Previous, r: &mut ::mingling::RenderResult) {
                // Call the original function
                #fn_name(#prev_param, r)
            }
        }

        // Keep the original function for internal use
        #(#fn_attrs)*
        #vis fn #fn_name(#prev_param: #previous_type, r: &mut ::mingling::RenderResult) {
            #fn_body
        }
    };

    expanded.into()
}
