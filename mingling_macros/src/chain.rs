//! Chain Attribute Macro Implementation
//!
//! This module provides the `#[chain(Group)]` attribute macro for automatically
//! generating structs that implement the `Chain` trait from async functions.

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Signature, Type, TypePath, parse_macro_input,
};

/// Extracts the previous type and parameter name from function arguments
fn extract_previous_info(sig: &Signature) -> syn::Result<(Pat, TypePath)> {
    // The function should have exactly one parameter
    if sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "Chain function must have exactly one parameter",
        ));
    }

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
                    "Parameter type must be a type path",
                )),
            }
        }
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "Chain function cannot have self parameter",
        )),
    }
}

/// Extracts the return type from the function signature
fn extract_return_type(sig: &Signature) -> syn::Result<TypePath> {
    match &sig.output {
        ReturnType::Type(_, ty) => match &**ty {
            Type::Path(type_path) => Ok(type_path.clone()),
            _ => Err(syn::Error::new(
                ty.span(),
                "Return type must be a type path",
            )),
        },
        ReturnType::Default => Err(syn::Error::new(
            sig.span(),
            "Chain function must have a return type",
        )),
    }
}

pub fn chain_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments (e.g., MyProgram from #[chain(MyProgram)])
    // If no argument is provided, use DefaultProgram
    let (group_name, use_crate_prefix) = if attr.is_empty() {
        (
            Ident::new("DefaultProgram", proc_macro2::Span::call_site()),
            true,
        )
    } else {
        (parse_macro_input!(attr as Ident), false)
    };

    // Parse the function item
    let input_fn = parse_macro_input!(item as ItemFn);

    // Validate the function
    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new(input_fn.sig.span(), "Chain function must be async")
            .to_compile_error()
            .into();
    }

    // Extract the previous type and parameter name from function arguments
    let (prev_param, previous_type) = match extract_previous_info(&input_fn.sig) {
        Ok(info) => info,
        Err(e) => return e.to_compile_error().into(),
    };

    // Extract the return type
    let return_type = match extract_return_type(&input_fn.sig) {
        Ok(ty) => ty,
        Err(e) => return e.to_compile_error().into(),
    };

    // Ensure the return type is named "GroupProcess"
    if return_type.path.segments.last().unwrap().ident != "GroupProcess" {
        return syn::Error::new(
            return_type.span(),
            "Return type must be 'mingling::marker::GroupProcess'",
        )
        .to_compile_error()
        .into();
    }

    // Get the function body
    let fn_body = &input_fn.block;

    // Get function attributes (excluding the chain attribute)
    let mut fn_attrs = input_fn.attrs.clone();

    // Remove any #[chain(...)] attributes to avoid infinite recursion
    fn_attrs.retain(|attr| !attr.path().is_ident("chain"));

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

            impl ::mingling::Chain<DefaultProgram> for #struct_name {
                type Previous = #previous_type;

                async fn proc(#prev_param: Self::Previous) ->
                    ::mingling::ChainProcess<DefaultProgram>
                {
                    let _ = GroupProcess;
                    // Call the original function
                    #fn_name(#prev_param).await
                }
            }

            // Keep the original function for internal use
            #(#fn_attrs)*
            #vis async fn #fn_name(#prev_param: #previous_type)
                -> ::mingling::ChainProcess<DefaultProgram>
            {
                #fn_body
            }
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #vis struct #struct_name;

            impl ::mingling::Chain<#group_name> for #struct_name {
                type Previous = #previous_type;

                async fn proc(#prev_param: Self::Previous) ->
                    ::mingling::ChainProcess<#group_name>
                {
                    let _ = GroupProcess;
                    // Call the original function
                    #fn_name(#prev_param).await
                }
            }

            // Keep the original function for internal use
            #(#fn_attrs)*
            #vis async fn #fn_name(#prev_param: #previous_type)
                -> ::mingling::ChainProcess<#group_name>
            {
                #fn_body
            }
        }
    };

    // Record the chain mapping
    let chain_entry = quote! {
        #struct_name => #previous_type,
    };

    let chain_exist_entry = quote! {
        Self::#previous_type => true,
    };

    let mut chains = crate::CHAINS.lock().unwrap();
    let mut chain_exist = crate::CHAINS_EXIST.lock().unwrap();
    let mut packed_types = crate::PACKED_TYPES.lock().unwrap();

    let chain_entry = chain_entry.to_string();
    let chain_exist_entry = chain_exist_entry.to_string();
    let previous_type_str = previous_type.to_token_stream().to_string();

    if !chains.contains(&chain_entry) {
        chains.push(chain_entry);
    }

    if !chain_exist.contains(&chain_exist_entry) {
        chain_exist.push(chain_exist_entry);
    }

    if !packed_types.contains(&previous_type_str) {
        packed_types.push(previous_type_str);
    }

    expanded.into()
}
