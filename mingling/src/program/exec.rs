#![allow(clippy::borrowed_box)]

use crate::{
    AnyOutput, ChainProcess, Dispatcher, Program, ProgramCollect, RenderResult,
    error::{ChainProcessError, ProgramInternalExecuteError},
    hint::{DispatcherNotFound, RendererNotFound},
};

pub mod error;

pub async fn exec<C: ProgramCollect>(
    program: Program<C>,
) -> Result<RenderResult, ProgramInternalExecuteError> {
    // Match user input
    let matched: (Box<dyn Dispatcher>, Vec<String>) = match match_user_input(&program) {
        Ok(r) => (r.0.clone(), r.1),
        Err(ProgramInternalExecuteError::DispatcherNotFound) => {
            // If no Dispatcher is found, dispatch to the DispatcherNotFound Dispatcher
            // to route it to the NoDispatcherFound struct
            let disp: Box<dyn Dispatcher> = Box::new(DispatcherNotFound);
            (disp, program.args)
        }
        Err(e) => return Err(e),
    };

    // Entry point
    let (dispatcher, args) = matched;
    let mut current = match handle_chain_process::<C>(dispatcher.begin(args)) {
        Ok(Next::RenderResult(render_result)) => return Ok(render_result),
        Ok(Next::AnyOutput(any)) => any,
        Err(e) => return Err(e),
    };

    loop {
        current = {
            // If a chain exists, execute as a chain
            if C::has_chain(&current) {
                match handle_chain_process::<C>(C::do_chain(current).await) {
                    Ok(Next::RenderResult(render_result)) => return Ok(render_result),
                    Ok(Next::AnyOutput(any)) => any,
                    Err(e) => return Err(e),
                }
            }
            // If no chain exists, attempt to render
            else if C::has_renderer(&current) {
                let mut render_result = RenderResult::default();
                C::render(current, &mut render_result);
                return Ok(render_result);
            }
            // If no renderer exists, transfer to the RendererNotFound Dispatcher for execution
            else {
                let disp: Box<dyn Dispatcher> = Box::new(RendererNotFound);

                match handle_chain_process::<C>(disp.begin(vec![format!("{:?}", current.type_id)]))
                {
                    Ok(Next::AnyOutput(any)) => any,
                    Ok(Next::RenderResult(result)) => return Ok(result),
                    Err(e) => return Err(e),
                }
            }
        };

        // If the dispatcher cannot find the next chain, end execution
        if C::has_chain(&current) {
            break;
        }
    }

    Ok(RenderResult::default())
}

/// Match user input against registered dispatchers and return the matched dispatcher and remaining arguments.
fn match_user_input<C: ProgramCollect>(
    program: &Program<C>,
) -> Result<(&Box<dyn Dispatcher>, Vec<String>), ProgramInternalExecuteError> {
    let nodes = get_nodes(program);
    let command = format!("{} ", program.args.join(" "));

    // Find all nodes that match the command prefix
    let matching_nodes: Vec<&(String, &Box<dyn Dispatcher>)> = nodes
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
            let trimmed_args: Vec<String> = program.args.iter().skip(prefix_len).cloned().collect();
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
            let trimmed_args: Vec<String> = program.args.iter().skip(prefix_len).cloned().collect();
            Ok((matched_prefix.1, trimmed_args))
        }
    }
}

#[inline(always)]
fn render<C: ProgramCollect>(any: AnyOutput) -> RenderResult {
    let mut render_result = RenderResult::default();
    C::render(any, &mut render_result);
    render_result
}

fn handle_chain_process<C: ProgramCollect>(
    process: ChainProcess,
) -> Result<Next, ProgramInternalExecuteError> {
    match process {
        Ok(any) => Ok(Next::AnyOutput(any)),
        Err(e) => match e {
            ChainProcessError::Broken(any_output) => {
                let render_result = render::<C>(any_output);
                Ok(Next::RenderResult(render_result))
            }
            _ => Err(e.into()),
        },
    }
}

// Get all registered dispatcher names from the program
fn get_nodes<C: ProgramCollect>(program: &Program<C>) -> Vec<(String, &Box<dyn Dispatcher>)> {
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
            (node_str, disp)
        })
        .collect()
}

enum Next {
    RenderResult(RenderResult),
    AnyOutput(AnyOutput),
}
