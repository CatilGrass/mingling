use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as SynResult, Token, Type};

/// Key-value attributes parsed from the macro input.
///
/// Currently supported attributes:
/// - `doc_hidden`: adds `#[doc(hidden)]` to the generated struct.
///
/// # Extending
/// Add a new field here, then handle it in `generate_struct_attrs`.
#[derive(Default)]
struct PackAttrs {
    attr_map: HashMap<String, bool>,
}

impl PackAttrs {
    fn doc_hidden(&self) -> bool {
        self.attr_map.get("doc_hidden").copied().unwrap_or(false)
    }
}

fn parse_attr_key_value(input: ParseStream) -> SynResult<(String, bool)> {
    let key: Ident = input.parse()?;
    input.parse::<Token![=]>()?;
    let lit: syn::LitBool = input.parse()?;
    Ok((key.to_string(), lit.value()))
}

enum PackInput {
    Explicit {
        group_name: syn::Path,
        type_name: Ident,
        inner_type: Type,
        attrs: PackAttrs,
    },
    Default {
        type_name: Ident,
        inner_type: Type,
        attrs: PackAttrs,
    },
}

impl Parse for PackInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Formats:
        //   - `Path, TypeName = InnerType`  → Explicit
        //   - `TypeName = InnerType`          → Default
        //
        // An optional trailing `, k = v, ...` is allowed after the inner type.

        if (input.peek(Ident) || input.peek(Token![crate]))
            && (input.peek2(Token![,]) || input.peek2(Token![::]))
        {
            // Explicit format
            let group_name = input.parse::<syn::Path>()?;
            input.parse::<Token![,]>()?;
            let type_name = input.parse()?;
            input.parse::<Token![=]>()?;
            let inner_type = input.parse()?;

            let attrs = if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                parse_attrs(input)?
            } else {
                PackAttrs::default()
            };

            Ok(PackInput::Explicit {
                group_name,
                type_name,
                inner_type,
                attrs,
            })
        } else if input.peek(Ident) && input.peek2(Token![=]) {
            // Default format
            let type_name = input.parse()?;
            input.parse::<Token![=]>()?;
            let inner_type = input.parse()?;

            let attrs = if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                parse_attrs(input)?
            } else {
                PackAttrs::default()
            };

            Ok(PackInput::Default {
                type_name,
                inner_type,
                attrs,
            })
        } else {
            Err(input.lookahead1().error())
        }
    }
}

/// Parse comma-separated key = value pairs, e.g. `doc_hidden = true`
fn parse_attrs(input: ParseStream) -> SynResult<PackAttrs> {
    let mut attr_map = HashMap::new();

    // Parse at least one attr
    let (key, value) = parse_attr_key_value(input)?;
    attr_map.insert(key, value);

    // Parse remaining `, key = value` pairs
    while input.peek(Token![,]) {
        let lookahead = input.fork();
        lookahead.parse::<Token![,]>()?;
        if lookahead.peek(Ident) && lookahead.peek2(Token![=]) {
            input.parse::<Token![,]>()?;
            let (key, value) = parse_attr_key_value(input)?;
            attr_map.insert(key, value);
        } else {
            break;
        }
    }

    Ok(PackAttrs { attr_map })
}

/// Given a `PackAttrs`, produce any additional `#[...]` attributes to place
/// before the struct definition.
///
/// # Extending
/// Add a new match arm here when you add a new attribute to `PackAttrs`.
fn generate_struct_attrs(attrs: &PackAttrs) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();

    if attrs.doc_hidden() {
        result.push(quote! { #[doc(hidden)] });
    }

    result
}

pub fn pack(input: TokenStream) -> TokenStream {
    // Parse the input
    let pack_input = syn::parse_macro_input!(input as PackInput);

    // Extract common fields and determine if we're using default or explicit group
    let (group_name, type_name, inner_type, attrs, use_default) = match pack_input {
        PackInput::Explicit {
            group_name,
            type_name,
            inner_type,
            attrs,
        } => (quote! { #group_name }, type_name, inner_type, attrs, false),
        PackInput::Default {
            type_name,
            inner_type,
            attrs,
        } => (
            crate::default_program_path(),
            type_name,
            inner_type,
            attrs,
            true,
        ),
    };

    // Build attribute tokens that should appear before `pub struct`
    let extra_attrs = generate_struct_attrs(&attrs);

    // Generate the struct definition
    #[cfg(not(feature = "general_renderer"))]
    let struct_def = quote! {
        #(#extra_attrs)*
        pub struct #type_name {
            pub(crate) inner: #inner_type,
        }
    };

    #[cfg(feature = "general_renderer")]
    let struct_def = quote! {
        #(#extra_attrs)*
        #[derive(serde::Serialize)]
        pub struct #type_name {
            pub(crate) inner: #inner_type,
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

    let register_impl = quote! {
        ::mingling::macros::register_type!(#type_name);
    };

    let any_out_impl = quote! {
        impl Into<mingling::AnyOutput<#group_name>> for #type_name {
            fn into(self) -> mingling::AnyOutput<#group_name> {
                mingling::AnyOutput::new(self)
            }
        }

        impl Into<mingling::ChainProcess<#group_name>> for #type_name {
            fn into(self) -> mingling::ChainProcess<#group_name> {
                mingling::AnyOutput::new(self).route_chain()
            }
        }

        impl #type_name {
            /// Converts the wrapper type into a `ChainProcess` for chaining operations.
            pub fn to_chain(self) -> mingling::ChainProcess<#group_name> {
                mingling::AnyOutput::new(self).route_chain()
            }

            /// Converts the wrapper type into a `ChainProcess` for rendering operations.
            pub fn to_render(self) -> mingling::ChainProcess<#group_name> {
                mingling::AnyOutput::new(self).route_renderer()
            }
        }
    };

    let group_impl = quote! {
        impl ::mingling::Groupped<#group_name> for #type_name {
            fn member_id() -> #group_name {
                #group_name::#type_name
            }
        }
    };

    // Combine all implementations
    let expanded = if use_default {
        // For default case, use ThisProgram
        quote! {
            #struct_def

            #new_impl
            #from_into_impl
            #as_ref_impl
            #deref_impl
            #default_impl
            #register_impl

            impl Into<mingling::AnyOutput<#group_name>> for #type_name {
                fn into(self) -> mingling::AnyOutput<#group_name> {
                    mingling::AnyOutput::new(self)
                }
            }

            impl From<#type_name> for mingling::ChainProcess<#group_name> {
                fn from(value: #type_name) -> Self {
                    mingling::AnyOutput::new(value).route_chain()
                }
            }

            impl #type_name {
                /// Converts the wrapper type into a `ChainProcess` for chaining operations.
                pub fn to_chain(self) -> mingling::ChainProcess<#group_name> {
                    mingling::AnyOutput::new(self).route_chain()
                }

                /// Converts the wrapper type into a `ChainProcess` for rendering operations.
                pub fn to_render(self) -> mingling::ChainProcess<#group_name> {
                    mingling::AnyOutput::new(self).route_renderer()
                }
            }

            impl ::mingling::Groupped<#group_name> for #type_name {
                fn member_id() -> #group_name {
                    #group_name::#type_name
                }
            }
        }
    } else {
        // For explicit case, use the provided group_name
        quote! {
            #struct_def

            #new_impl
            #from_into_impl
            #as_ref_impl
            #deref_impl
            #default_impl
            #register_impl

            #any_out_impl
            #group_impl
        }
    };

    expanded.into()
}
