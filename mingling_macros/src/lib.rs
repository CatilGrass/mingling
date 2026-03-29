//! Mingling Macros Crate
//!
//! This crate provides procedural macros for the Mingling framework.
//! Macros are implemented in separate modules and re-exported here.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

mod chain;
mod chain_struct;
mod dispatcher_chain;
mod node;
mod render;
mod renderer;

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global variable declarations for storing chain and renderer mappings
pub(crate) static CHAINS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static RENDERERS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static CHAINS_EXIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub(crate) static RENDERERS_EXIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Creates a command node from a dot-separated path string.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::node;
///
/// let node = node!("root.subcommand.action");
/// ```
#[proc_macro]
pub fn node(input: TokenStream) -> TokenStream {
    node::node(input)
}

/// Macro for creating wrapper types with automatic trait implementations.
///
/// This macro creates a new struct that wraps an inner type and automatically
/// implements common traits:
/// - `From<InnerType>` and `Into<InnerType>`
/// - `new()` constructor
/// - `Default` (if inner type implements Default)
/// - `AsRef<InnerType>` and `AsMut<InnerType>`
/// - `Deref` and `DerefMut` to inner type
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::chain_struct;
///
/// // Creates a wrapper type around String
/// chain_struct!(NameString = String);
///
/// // Usage:
/// let name = NameString::new("Hello".to_string());
/// let inner: String = name.into(); // Into conversion
/// let name2 = NameString::from("World".to_string()); // From conversion
/// let ref_str: &String = name2.as_ref(); // AsRef
/// ```
#[proc_macro]
pub fn chain_struct(input: TokenStream) -> TokenStream {
    chain_struct::chain_struct(input)
}

#[proc_macro]
pub fn dispatcher(input: TokenStream) -> TokenStream {
    dispatcher_chain::dispatcher_chain(input)
}

#[proc_macro]
pub fn dispatcher_render(input: TokenStream) -> TokenStream {
    dispatcher_chain::dispatcher_render(input)
}

/// Macro for printing to a RenderResult without newline.
///
/// This macro expands to a call to `RenderResult::print` with formatted arguments.
/// It expects a mutable reference to a `RenderResult` named `r` to be in scope.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::r_print;
///
/// let mut r = RenderResult::default();
/// r_print!("Hello, {}!", "world");
/// ```
#[proc_macro]
pub fn r_print(input: TokenStream) -> TokenStream {
    render::r_print(input)
}

/// Macro for printing to a RenderResult with newline.
///
/// This macro expands to a call to `RenderResult::println` with formatted arguments.
/// It expects a mutable reference to a `RenderResult` named `r` to be in scope.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::r_println;
///
/// let mut r = RenderResult::default();
/// r_println!("Hello, {}!", "world");
/// ```
#[proc_macro]
pub fn r_println(input: TokenStream) -> TokenStream {
    render::r_println(input)
}

/// Attribute macro for automatically generating structs that implement the `Chain` trait.
///
/// This macro transforms an async function into a struct that implements
/// the `Chain` trait. The struct name is specified in the attribute.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::chain;
///
/// #[chain]
/// pub async fn init_entry(_: InitBegin) -> mingling::ChainProcess {
///     AnyOutput::new::<InitResult>("Init!".to_string().into()).route_chain()
/// }
/// ```
///
/// This generates:
/// ```ignore
/// pub struct InitEntry;
/// impl Chain for InitEntry {
///     type Previous = InitBegin;
///     async fn proc(_: Self::Previous) -> mingling::ChainProcess {
///         AnyOutput::new::<InitResult>("Init!".to_string().into()).route_chain()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn chain(_attr: TokenStream, item: TokenStream) -> TokenStream {
    chain::chain_attr(item)
}

/// Attribute macro for automatically generating structs that implement the `Renderer` trait.
///
/// This macro transforms a function into a struct that implements
/// the `Renderer` trait. The struct name is specified in the attribute.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::renderer;
///
/// #[renderer]
/// fn init_result_render(p: InitResult) {
///     let str: String = p.into();
///     r_println!("{}", str);
/// }
/// ```
///
/// This generates:
/// ```ignore
/// pub struct InitResultRender;
/// impl Renderer for InitResultRender {
///     type Previous = InitResult;
///
///     fn render(p: Self::Previous, r: &mut RenderResult) {
///         let str: String = p.into();
///         r_println!("{}", str);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn renderer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    renderer::renderer_attr(item)
}

/// Macro for creating a program structure that collects all chains and renderers.
///
/// This macro creates a struct that implements the `ProgramCollect` trait,
/// which collects all chains and renderers registered with `#[chain]` and `#[renderer]`
/// attribute macros. The program can then be used to execute the command chain.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::program;
///
/// program!(MyProgram);
///
/// // This generates:
/// pub struct MyProgram;
/// impl mingling::ProgramCollect for MyProgram {
///     mingling::__dispatch_program_renderers!(...);
///     mingling::__dispatch_program_chains!(...);
/// }
/// impl MyProgram {
///     pub fn new() -> mingling::Program<MyProgram> {
///         mingling::Program::new()
///     }
/// }
/// ```
#[proc_macro]
pub fn program(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let renderers = RENDERERS.lock().unwrap().clone();
    let chains = CHAINS.lock().unwrap().clone();
    let renderer_exist = RENDERERS_EXIST.lock().unwrap().clone();
    let chain_exist = CHAINS_EXIST.lock().unwrap().clone();

    let renderer_tokens: Vec<proc_macro2::TokenStream> = renderers
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let chain_tokens: Vec<proc_macro2::TokenStream> = chains
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let renderer_exist_tokens: Vec<proc_macro2::TokenStream> = renderer_exist
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let chain_exist_tokens: Vec<proc_macro2::TokenStream> = chain_exist
        .iter()
        .map(|s| syn::parse_str::<proc_macro2::TokenStream>(s).unwrap())
        .collect();

    let expanded = quote! {
        pub struct #name;

        impl ::mingling::ProgramCollect for #name {
            ::mingling::__dispatch_program_renderers!(
                #(#renderer_tokens)*
            );
            ::mingling::__dispatch_program_chains!(
                #(#chain_tokens)*
            );
            fn has_renderer(any: &::mingling::AnyOutput) -> bool {
                match any.type_id {
                    #(#renderer_exist_tokens)*
                    _ => false
                }
            }
            fn has_chain(any: &::mingling::AnyOutput) -> bool {
                match any.type_id {
                    #(#chain_exist_tokens)*
                    _ => false
                }
            }
        }

        impl #name {
            pub fn new() -> ::mingling::Program<#name> {
                ::mingling::Program::new()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Internal macro for registering chains.
///
/// This macro is used internally by the `#[chain]` attribute macro
/// and should not be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __register_chain(input: TokenStream) -> TokenStream {
    let chain_entry = parse_macro_input!(input as syn::LitStr);
    let entry_str = chain_entry.value();

    CHAINS.lock().unwrap().push(entry_str);

    TokenStream::new()
}

/// Internal macro for registering renderers.
///
/// This macro is used internally by the `#[renderer]` attribute macro
/// and should not be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __register_renderer(input: TokenStream) -> TokenStream {
    let renderer_entry = parse_macro_input!(input as syn::LitStr);
    let entry_str = renderer_entry.value();

    RENDERERS.lock().unwrap().push(entry_str);

    TokenStream::new()
}
