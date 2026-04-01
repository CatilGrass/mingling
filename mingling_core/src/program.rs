use crate::{
    AnyOutput, ChainProcess, RenderResult, asset::dispatcher::Dispatcher,
    error::ProgramExecuteError,
};
use std::{env, fmt::Display, pin::Pin};

#[doc(hidden)]
pub mod exec;
#[doc(hidden)]
pub mod setup;

mod config;
pub use config::*;

mod flag;
pub use flag::*;
use tokio::io::AsyncWriteExt;

/// Program, used to define the behavior of the entire command-line program
#[derive(Default)]
pub struct Program<C, G>
where
    C: ProgramCollect,
    G: Display,
{
    pub(crate) collect: std::marker::PhantomData<C>,
    pub(crate) group: std::marker::PhantomData<G>,

    pub(crate) args: Vec<String>,
    pub(crate) dispatcher: Vec<Box<dyn Dispatcher<G>>>,

    pub stdout_setting: ProgramStdoutSetting,
    pub user_context: ProgramUserContext,
}

impl<C, G> Program<C, G>
where
    C: ProgramCollect<Enum = G>,
    G: Display,
{
    /// Creates a new Program instance, initializing args from environment.
    pub fn new() -> Self {
        Program {
            collect: std::marker::PhantomData,
            group: std::marker::PhantomData,
            args: env::args().collect(),
            dispatcher: Vec::new(),
            stdout_setting: Default::default(),
            user_context: Default::default(),
        }
    }

    /// Run the command line program
    pub async fn exec_without_render(mut self) -> Result<RenderResult, ProgramExecuteError> {
        self.args = self.args.iter().skip(1).cloned().collect();
        crate::exec::exec(self).await.map_err(|e| e.into())
    }

    /// Run the command line program
    pub async fn exec(self) {
        let stdout_setting = self.stdout_setting.clone();
        let result = match self.exec_without_render().await {
            Ok(r) => r,
            Err(e) => match e {
                ProgramExecuteError::DispatcherNotFound => {
                    eprintln!("Dispatcher not found");
                    return;
                }
                ProgramExecuteError::RendererNotFound(renderer_name) => {
                    eprintln!("Renderer `{}` not found", renderer_name);
                    return;
                }
                ProgramExecuteError::Other(e) => {
                    eprintln!("{}", e);
                    return;
                }
            },
        };

        // Render result
        if stdout_setting.render_output && !result.is_empty() {
            print!("{}", result);
            if let Err(e) = tokio::io::stdout().flush().await
                && stdout_setting.error_output
            {
                eprintln!("{}", e);
            }
        }
    }
}

/// Collected program context
///
/// Note: It is recommended to use the `gen_program!()` macro from [mingling_macros](https://crates.io/crates/mingling_macros) to automatically create this type
pub trait ProgramCollect {
    /// Enum type representing internal IDs for the program
    type Enum: Display;

    /// Build an [AnyOutput](./struct.AnyOutput.html) to indicate that a renderer was not found
    fn build_renderer_not_found(member_id: Self::Enum) -> AnyOutput<Self::Enum>;

    /// Build an [AnyOutput](./struct.AnyOutput.html) to indicate that a dispatcher was not found
    fn build_dispatcher_not_found(args: Vec<String>) -> AnyOutput<Self::Enum>;

    /// Render the input [AnyOutput](./struct.AnyOutput.html)
    fn render(any: AnyOutput<Self::Enum>, r: &mut RenderResult);

    /// Find a matching chain to continue execution based on the input [AnyOutput](./struct.AnyOutput.html), returning a new [AnyOutput](./struct.AnyOutput.html)
    fn do_chain(
        any: AnyOutput<Self::Enum>,
    ) -> Pin<Box<dyn Future<Output = ChainProcess<Self::Enum>> + Send>>;

    /// Whether the program has a renderer that can handle the current [AnyOutput](./struct.AnyOutput.html)
    fn has_renderer(any: &AnyOutput<Self::Enum>) -> bool;

    /// Whether the program has a chain that can handle the current [AnyOutput](./struct.AnyOutput.html)
    fn has_chain(any: &AnyOutput<Self::Enum>) -> bool;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dispatch_program_renderers {
    (
        $( $render_ty:ty => $prev_ty:ident, )*
    ) => {
        fn render(any: mingling::AnyOutput<Self::Enum>, r: &mut mingling::RenderResult) {
            match any.member_id {
                $(
                    Self::$prev_ty => {
                        // SAFETY: The `type_id` check ensures that `any` contains a value of type `$prev_ty`,
                        // so downcasting to `$prev_ty` is safe.
                        let value = unsafe { any.downcast::<$prev_ty>().unwrap_unchecked() };
                        <$render_ty as mingling::Renderer>::render(value, r);
                    }
                )*
                _ => (),
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dispatch_program_chains {
    (
        $( $chain_ty:ty => $chain_prev:ident, )*
    ) => {
        fn do_chain(
            any: mingling::AnyOutput<Self::Enum>,
        ) -> std::pin::Pin<Box<dyn Future<Output = mingling::ChainProcess<Self::Enum>> + Send>> {
            match any.member_id {
                $(
                    Self::$chain_prev => {
                        // SAFETY: The `type_id` check ensures that `any` contains a value of type `$chain_prev`,
                        // so downcasting to `$chain_prev` is safe.
                        let value = unsafe { any.downcast::<$chain_prev>().unwrap_unchecked() };
                        let fut = async { <$chain_ty as mingling::Chain<Self::Enum>>::proc(value).await };
                        Box::pin(fut)
                    }
                )*
                _ => panic!("No chain found for type id: {:?}", any.type_id),
            }
        }
    };
}
