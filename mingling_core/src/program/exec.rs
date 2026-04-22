#![allow(clippy::borrowed_box)]

use crate::{
    AnyOutput, ChainProcess, Dispatcher, Next, Program, ProgramCollect, RenderResult,
    error::ProgramInternalExecuteError,
};

#[doc(hidden)]
pub mod error;

#[cfg(feature = "async")]
pub async fn exec<C, G>(
    program: &Program<C, G>,
) -> Result<RenderResult, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
{
    let mut current = dispatch_args_dynamic(program, program.args.clone())?;
    let mut stop_next = false;

    loop {
        let final_exec = stop_next;

        current = {
            // If a chain exists, execute as a chain
            if C::has_chain(&current) {
                match C::do_chain(current).await {
                    ChainProcess::Ok((any, Next::Renderer)) => {
                        return Ok(render::<C, G>(program, any));
                    }
                    ChainProcess::Ok((any, Next::Chain)) => any,
                    ChainProcess::Err(e) => return Err(e.into()),
                }
            }
            // If no chain exists, attempt to render
            else if C::has_renderer(&current) {
                return Ok(render::<C, G>(program, current));
            }
            // No renderer exists
            else {
                stop_next = true;
                C::build_renderer_not_found(current.member_id)
            }
        };

        if final_exec && stop_next {
            break;
        }
    }
    Ok(RenderResult::default())
}

#[cfg(not(feature = "async"))]
pub fn exec<C, G>(program: &Program<C, G>) -> Result<RenderResult, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
{
    let mut current = dispatch_args_dynamic(program, program.args.clone())?;
    let mut stop_next = false;

    loop {
        let final_exec = stop_next;

        current = {
            // If a chain exists, execute as a chain
            if C::has_chain(&current) {
                match C::do_chain(current) {
                    ChainProcess::Ok((any, Next::Renderer)) => {
                        return Ok(render::<C, G>(program, any));
                    }
                    ChainProcess::Ok((any, Next::Chain)) => any,
                    ChainProcess::Err(e) => return Err(e.into()),
                }
            }
            // If no chain exists, attempt to render
            else if C::has_renderer(&current) {
                return Ok(render::<C, G>(program, current));
            }
            // No renderer exists
            else {
                stop_next = true;
                C::build_renderer_not_found(current.member_id)
            }
        };

        if final_exec && stop_next {
            break;
        }
    }
    Ok(RenderResult::default())
}

/// Dynamically dispatch input arguments to registered entry types
pub(crate) fn dispatch_args_dynamic<C, G>(
    program: &Program<C, G>,
    args: Vec<String>,
) -> Result<AnyOutput<G>, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
{
    let next = match match_user_input(program, args) {
        Ok((dispatcher, args)) => {
            // Entry point
            match dispatcher.begin(args) {
                ChainProcess::Ok((any, _)) => any,
                ChainProcess::Err(e) => return Err(e.into()),
            }
        }
        Err(ProgramInternalExecuteError::DispatcherNotFound) => {
            // No matching Dispatcher is found
            C::build_dispatcher_not_found(program.args.clone())
        }
        Err(e) => return Err(e),
    };
    Ok(next)
}

/// Match user input against registered dispatchers and return the matched dispatcher and remaining arguments.
#[allow(clippy::type_complexity)]
pub(crate) fn match_user_input<C, G>(
    program: &Program<C, G>,
    args: Vec<String>,
) -> Result<(&(dyn Dispatcher<G> + Send + Sync), Vec<String>), ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
{
    let nodes = program.get_nodes();
    let command = format!("{} ", args.join(" "));

    // Find all nodes that match the command prefix
    let matching_nodes: Vec<&(String, &(dyn Dispatcher<G> + Send + Sync))> = nodes
        .iter()
        // Also add a space to the node string to ensure consistent matching logic
        .filter(|(node_str, _)| command.starts_with(&format!("{} ", node_str)))
        .collect();

    match matching_nodes.len() {
        0 => {
            // No matching node found
            Err(ProgramInternalExecuteError::DispatcherNotFound)
        }
        1 => {
            let matched_prefix = matching_nodes[0];
            let prefix_len = matched_prefix.0.split_whitespace().count();
            let trimmed_args: Vec<String> = args.iter().skip(prefix_len).cloned().collect();
            Ok((matched_prefix.1, trimmed_args))
        }
        _ => {
            // Multiple matching nodes found
            // Find the node with the longest length (most specific match)
            let matched_prefix = matching_nodes
                .iter()
                .max_by_key(|node| node.0.len())
                .unwrap();

            let prefix_len = matched_prefix.0.split_whitespace().count();
            let trimmed_args: Vec<String> = args.iter().skip(prefix_len).cloned().collect();
            Ok((matched_prefix.1, trimmed_args))
        }
    }
}

#[inline(always)]
#[allow(unused_variables)]
fn render<C: ProgramCollect<Enum = G>, G>(
    program: &Program<C, G>,
    any: AnyOutput<G>,
) -> RenderResult {
    #[cfg(not(feature = "general_renderer"))]
    {
        let mut render_result = RenderResult::default();
        C::render(any, &mut render_result);
        render_result
    }
    #[cfg(feature = "general_renderer")]
    {
        match program.general_renderer_name {
            super::GeneralRendererSetting::Disable => {
                let mut render_result = RenderResult::default();
                C::render(any, &mut render_result);
                render_result
            }
            _ => C::general_render(any, &program.general_renderer_name).unwrap(),
        }
    }
}
