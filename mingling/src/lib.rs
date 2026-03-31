// Re-Export Core lib
pub use mingling::*;
pub use mingling_core as mingling;

#[cfg(feature = "parser")]
pub mod parser;

#[allow(unused_imports)]
pub mod macros {
    pub use mingling_macros::chain;
    pub use mingling_macros::chain_struct;
    pub use mingling_macros::dispatcher;
    pub use mingling_macros::dispatcher_render;
    pub use mingling_macros::node;
    pub use mingling_macros::program;
    pub use mingling_macros::r_print;
    pub use mingling_macros::r_println;
    pub use mingling_macros::renderer;
}
