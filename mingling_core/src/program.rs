#[cfg(feature = "comp")]
use crate::{ShellContext, Suggest};

#[cfg(feature = "general_renderer")]
use crate::error::GeneralRendererSerializeError;

use crate::{
    AnyOutput, ChainProcess, RenderResult, asset::dispatcher::Dispatcher,
    error::ProgramExecuteError,
};
use std::{env, fmt::Display, pin::Pin, sync::OnceLock};

#[doc(hidden)]
pub mod exec;
#[doc(hidden)]
pub mod setup;

mod config;
pub use config::*;

mod flag;
pub use flag::*;
use tokio::io::AsyncWriteExt;

/// Global static reference to the current program instance
static THIS_PROGRAM: OnceLock<Option<Box<dyn std::any::Any + Send + Sync>>> = OnceLock::new();

/// Returns a reference to the current program instance, panics if not set.
pub fn this<C>() -> &'static Program<C, C>
where
    C: ProgramCollect + Display + 'static,
{
    try_get_this_program().expect("Program not initialized")
}

/// Returns a reference to the current program instance, if set.
fn try_get_this_program<C>() -> Option<&'static Program<C, C>>
where
    C: ProgramCollect + Display + 'static,
{
    THIS_PROGRAM
        .get()?
        .as_ref()?
        .downcast_ref::<Program<C, C>>()
}

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
    pub(crate) dispatcher: Vec<Box<dyn Dispatcher<G> + Send + Sync>>,

    pub stdout_setting: ProgramStdoutSetting,
    pub user_context: ProgramUserContext,

    #[cfg(feature = "general_renderer")]
    pub general_renderer_name: GeneralRendererSetting,
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

            #[cfg(feature = "general_renderer")]
            general_renderer_name: GeneralRendererSetting::Disable,
        }
    }

    /// Returns a reference to the current program instance, if set.
    pub async fn this_program() -> &'static Program<C, G>
    where
        C: 'static,
        G: 'static,
    {
        THIS_PROGRAM
            .get()
            .unwrap()
            .as_ref()
            .unwrap()
            .downcast_ref::<Program<C, G>>()
            .unwrap()
    }

    /// Sets the current program instance and runs the provided async function.
    async fn set_instance_and_run<F, Fut>(self, f: F) -> Fut::Output
    where
        C: 'static + Send + Sync,
        G: 'static + Send + Sync,
        F: FnOnce(&'static Program<C, G>) -> Fut + Send + Sync,
        Fut: Future + Send,
    {
        THIS_PROGRAM.get_or_init(|| Some(Box::new(self)));
        let program = THIS_PROGRAM
            .get()
            .unwrap()
            .as_ref()
            .unwrap()
            .downcast_ref::<Program<C, G>>()
            .unwrap();
        f(program).await
    }

    /// Run the command line program
    pub async fn exec_without_render(mut self) -> Result<RenderResult, ProgramExecuteError>
    where
        C: 'static + Send + Sync,
        G: 'static + Send + Sync,
    {
        self.args = self.args.iter().skip(1).cloned().collect();
        self.set_instance_and_run(|p| async { crate::exec::exec(p).await.map_err(|e| e.into()) })
            .await
    }

    /// Run the command line program
    pub async fn exec(self)
    where
        C: 'static + Send + Sync,
        G: 'static + Send + Sync,
    {
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

    // Get all registered dispatcher names from the program
    pub fn get_nodes(&self) -> Vec<(String, &(dyn Dispatcher<G> + Send + Sync))> {
        get_nodes(self)
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

    /// Match and execute specific completion logic based on any Entry
    #[cfg(feature = "comp")]
    fn do_comp(any: &AnyOutput<Self::Enum>, ctx: &ShellContext) -> Suggest;

    /// Whether the program has a renderer that can handle the current [AnyOutput](./struct.AnyOutput.html)
    fn has_renderer(any: &AnyOutput<Self::Enum>) -> bool;

    /// Whether the program has a chain that can handle the current [AnyOutput](./struct.AnyOutput.html)
    fn has_chain(any: &AnyOutput<Self::Enum>) -> bool;

    /// Perform general rendering and presentation of any type
    #[cfg(feature = "general_renderer")]
    fn general_render(
        any: AnyOutput<Self::Enum>,
        setting: &GeneralRendererSetting,
    ) -> Result<RenderResult, GeneralRendererSerializeError>;
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

// Get all registered dispatcher names from the program
pub fn get_nodes<C: ProgramCollect<Enum = G>, G: Display>(
    program: &Program<C, G>,
) -> Vec<(String, &(dyn Dispatcher<G> + Send + Sync))> {
    program
        .dispatcher
        .iter()
        .map(|disp| {
            let node_str = disp
                .node()
                .to_string()
                .split('.')
                .collect::<Vec<_>>()
                .join(" ");
            (node_str, &**disp)
        })
        .collect()
}
