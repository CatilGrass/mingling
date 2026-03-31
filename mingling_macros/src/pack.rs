//! Chain Struct Macro Implementation
//!
//! This module provides the `pack!` macro for creating wrapper types
//! with automatic implementations of common traits.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as SynResult, Token, Type};

/// Parses input in the format: `TypeName = InnerType`
struct PackInput {
    type_name: Ident,
    inner_type: Type,
}

impl Parse for PackInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let type_name = input.parse()?;
        input.parse::<Token![=]>()?;
        let inner_type = input.parse()?;

        Ok(PackInput {
            type_name,
            inner_type,
        })
    }
}

pub fn pack(input: TokenStream) -> TokenStream {
    let PackInput {
        type_name,
        inner_type,
    } = syn::parse_macro_input!(input as PackInput);

    // Generate the struct definition
    #[cfg(not(feature = "serde"))]
    let struct_def = quote! {
        #[derive(Debug)]
        pub struct #type_name {
            inner: #inner_type,
        }
    };

    #[cfg(feature = "serde")]
    let struct_def = quote! {
        #[derive(Debug, serde::Serialize)]
        pub struct #type_name {
            inner: #inner_type,
        }
    };

    // Generate the new() method
    let new_impl = quote! {
        impl #type_name {
            /// Creates a new instance of the wrapper type
            pub fn new(inner: #inner_type) -> Self {
                Self { inner }
            }
        }
    };

    // Generate From and Into implementations
    let from_into_impl = quote! {
        impl From<#inner_type> for #type_name {
            fn from(inner: #inner_type) -> Self {
                Self::new(inner)
            }
        }

        impl From<#type_name> for #inner_type {
            fn from(wrapper: #type_name) -> #inner_type {
                wrapper.inner
            }
        }
    };

    // Generate AsRef and AsMut implementations
    let as_ref_impl = quote! {
        impl ::std::convert::AsRef<#inner_type> for #type_name {
            fn as_ref(&self) -> &#inner_type {
                &self.inner
            }
        }

        impl ::std::convert::AsMut<#inner_type> for #type_name {
            fn as_mut(&mut self) -> &mut #inner_type {
                &mut self.inner
            }
        }
    };

    // Generate Deref and DerefMut implementations
    let deref_impl = quote! {
        impl ::std::ops::Deref for #type_name {
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::std::ops::DerefMut for #type_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };

    // Check if the inner type implements Default by generating conditional code
    let default_impl = quote! {
        impl ::std::default::Default for #type_name
        where
            #inner_type: ::std::default::Default,
        {
            fn default() -> Self {
                Self::new(::std::default::Default::default())
            }
        }
    };

    let any_out_impl = quote! {
        impl Into<mingling::AnyOutput> for #type_name {
            fn into(self) -> mingling::AnyOutput {
                mingling::AnyOutput::new(self)
            }
        }

        impl Into<mingling::ChainProcess> for #type_name {
            fn into(self) -> mingling::ChainProcess {
                mingling::AnyOutput::new(self).route_chain()
            }
        }

        impl #type_name {
            /// Converts the wrapper type into a `ChainProcess` for chaining operations.
            pub fn to_chain(self) -> mingling::ChainProcess {
                mingling::AnyOutput::new(self).route_chain()
            }

            /// Converts the wrapper type into a `ChainProcess` for rendering operations.
            pub fn to_render(self) -> mingling::ChainProcess {
                mingling::AnyOutput::new(self).route_renderer()
            }
        }
    };

    // Combine all implementations
    let expanded = quote! {
        #struct_def

        #new_impl
        #from_into_impl
        #as_ref_impl
        #deref_impl
        #default_impl

        #any_out_impl
    };

    expanded.into()
}
