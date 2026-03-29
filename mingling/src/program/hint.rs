use crate::{AnyOutput, ChainProcess, Dispatcher, Node};

/// Marker: Program End
///
/// If a chain outputs ProgramEnd to the Chain,
/// the program will terminate directly.
///
/// You can implement Renderer for ProgramEnd
/// to render relevant information after the program ends.
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct ProgramEnd;

/// Marker: Chain Not Found
///
/// If a Chain or Dispatcher outputs NoChainFound to the Chain,
/// the program will terminate directly.
///
/// You can implement Renderer for NoChainFound
/// to render relevant information when a Chain cannot be found.
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct NoChainFound {
    pub name: String,
}

/// Marker: Dispatcher Not Found
///
/// If a Dispatcher outputs NoDispatcherFound to the Chain,
/// the program will terminate directly.
///
/// You can implement Renderer for NoDispatcherFound
/// to render relevant information when a Dispatcher cannot be found.
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct NoDispatcherFound {
    pub args: Vec<String>,
}

#[derive(Default)]
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct DispatcherNotFound;
impl Dispatcher for DispatcherNotFound {
    fn node(&self) -> crate::Node {
        Node::default().join("_not_found")
    }

    fn begin(&self, args: Vec<String>) -> ChainProcess {
        AnyOutput::new(NoDispatcherFound { args }).route_renderer()
    }

    fn clone_dispatcher(&self) -> Box<dyn Dispatcher> {
        Box::new(DispatcherNotFound)
    }
}

/// Marker: Renderer Not Found
///
/// If a Chain outputs NoRendererFound to the Chain,
/// the program will terminate directly.
///
/// You can implement Renderer for NoRendererFound
/// to render relevant information when a Renderer cannot be found.
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct NoRendererFound {
    pub type_to_render: String,
}

#[derive(Default)]
#[cfg_attr(feature = "serde_renderer", derive(serde::Serialize))]
pub struct RendererNotFound;
impl Dispatcher for RendererNotFound {
    fn node(&self) -> crate::Node {
        Node::default().join("_not_found")
    }

    fn begin(&self, args: Vec<String>) -> ChainProcess {
        AnyOutput::new(NoRendererFound {
            type_to_render: args.first().unwrap().clone(),
        })
        .route_renderer()
    }

    fn clone_dispatcher(&self) -> Box<dyn Dispatcher> {
        Box::new(RendererNotFound)
    }
}
