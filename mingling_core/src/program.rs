use crate::{
    AnyOutput, ChainProcess, RenderResult, asset::dispatcher::Dispatcher,
    error::ProgramExecuteError,
};
use std::{env, pin::Pin};

pub mod exec;
pub mod hint;
pub mod setup;

mod config;
pub use config::*;

mod flag;
pub use flag::*;
use tokio::io::AsyncWriteExt;

#[derive(Default)]
pub struct Program<C: ProgramCollect> {
    pub(crate) collect: std::marker::PhantomData<C>,

    pub(crate) args: Vec<String>,
    pub(crate) dispatcher: Vec<Box<dyn Dispatcher>>,

    pub stdout_setting: ProgramStdoutSetting,
    pub user_context: ProgramUserContext,
}

impl<C> Program<C>
where
    C: ProgramCollect,
{
    /// Creates a new Program instance, initializing args from environment.
    pub fn new() -> Self {
        Program {
            collect: std::marker::PhantomData,
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

pub trait ProgramCollect {
    fn render(any: AnyOutput, r: &mut RenderResult);
    fn do_chain(any: AnyOutput) -> Pin<Box<dyn Future<Output = ChainProcess> + Send>>;
    fn has_renderer(any: &AnyOutput) -> bool;
    fn has_chain(any: &AnyOutput) -> bool;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dispatch_program_renderers {
    (
        $( $render_ty:ty => $prev_ty:ty, )*
    ) => {
        fn render(any: mingling::AnyOutput, r: &mut mingling::RenderResult) {
            match any.type_id {
                $(
                    id if id == std::any::TypeId::of::<$prev_ty>() => {
                        // SAFETY: The `type_id` check ensures that `any` contains a value of type `$chain_prev`,
                        // so downcasting to `$chain_prev` is safe.
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
        $( $chain_ty:ty => $chain_prev:ty, )*
    ) => {
        fn do_chain(
            any: mingling::AnyOutput,
        ) -> std::pin::Pin<Box<dyn Future<Output = mingling::ChainProcess> + Send>> {
            match any.type_id {
                $(
                    id if id == std::any::TypeId::of::<$chain_prev>() => {
                        // SAFETY: The `type_id` check ensures that `any` contains a value of type `$chain_prev`,
                        // so downcasting to `$chain_prev` is safe.
                        let value = unsafe { any.downcast::<$chain_prev>().unwrap_unchecked() };
                        let fut = async { <$chain_ty as mingling::Chain>::proc(value).await };
                        Box::pin(fut)
                    }
                )*
                _ => panic!("No chain found for type id: {:?}", any.type_id),
            }
        }
    };
}
