//! Mingling Macros Crate
//!
//! This crate provides procedural macros for the Mingling framework.
//! Macros are implemented in separate modules and re-exported here.

use proc_macro::TokenStream;

mod chain;
mod chain_struct;
mod dispatcher;
mod node;
mod render;
mod renderer;

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

/// Derive macro for automatically implementing the `Dispatcher` trait.
///
/// This macro generates an implementation of `mingling::Dispatcher` for a struct.
/// By default, it uses the struct name converted to snake_case as the command path.
/// You can also specify a custom path using the `#[dispatcher("path")]` attribute.
///
/// # Examples
///
/// ```ignore
/// use mingling_macros::Dispatcher;
///
/// // Uses default path: "remote.add"
/// #[derive(Dispatcher)]
/// pub struct RemoteAdd;
///
/// // Uses custom path: "remote.rm"
/// #[derive(Dispatcher)]
/// #[dispatcher("remote.rm")]
/// pub struct MyCommand;
/// ```
#[proc_macro_derive(Dispatcher, attributes(dispatcher))]
pub fn dispatcher_derive(input: TokenStream) -> TokenStream {
    dispatcher::dispatcher_derive(input)
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
/// #[chain(InitEntry)]
/// pub async fn proc(_: InitBegin) -> mingling::AnyOutput {
///     AnyOutput::new::<InitResult>("初始化成功！".to_string().into())
/// }
/// ```
///
/// This generates:
/// ```ignore
/// pub struct InitEntry;
/// impl Chain for InitEntry {
///     type Previous = InitBegin;
///     async fn proc(_: Self::Previous) -> mingling::AnyOutput {
///         AnyOutput::new::<InitResult>("初始化成功！".to_string().into())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn chain(attr: TokenStream, item: TokenStream) -> TokenStream {
    chain::chain_attr(attr, item)
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
/// #[renderer(InitResultRenderer)]
/// fn render(p: InitResult, r: &mut RenderResult) {
///     let str: String = p.into();
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
///     fn render(p: Self::Previous, r: &mut RenderResult) {
///         let str: String = p.into();
///         r_println!("{}", str);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn renderer(attr: TokenStream, item: TokenStream) -> TokenStream {
    renderer::renderer_attr(attr, item)
}
