#![allow(clippy::borrowed_box)]

use std::fmt::Display;

use crate::{
    AnyOutput, ChainProcess, Dispatcher, Next, Program, ProgramCollect, RenderResult,
    error::ProgramInternalExecuteError,
};

#[doc(hidden)]
pub mod error;

pub async fn exec<C, G>(program: Program<C, G>) -> Result<RenderResult, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
    G: Display,
{
    let mut current;
    let mut stop_next = false;

    // Match user input
    match match_user_input(&program) {
        Ok((dispatcher, args)) => {
            // Entry point
            current = match dispatcher.begin(args) {
                ChainProcess::Ok((any, Next::Renderer)) => return Ok(render::<C, G>(any)),
                ChainProcess::Ok((any, Next::Chain)) => any,
                ChainProcess::Err(e) => return Err(e.into()),
            };
        }
        Err(ProgramInternalExecuteError::DispatcherNotFound) => {
            // No matching Dispatcher is found
            current = C::build_dispatcher_not_found(program.args);
        }
        Err(e) => return Err(e),
    };

    loop {
        let final_exec = stop_next;

        current = {
            // If a chain exists, execute as a chain
            if C::has_chain(&current) {
                match C::do_chain(current).await {
                    ChainProcess::Ok((any, Next::Renderer)) => return Ok(render::<C, G>(any)),
                    ChainProcess::Ok((any, Next::Chain)) => any,
                    ChainProcess::Err(e) => return Err(e.into()),
                }
            }
            // If no chain exists, attempt to render
            else if C::has_renderer(&current) {
                let mut render_result = RenderResult::default();
                C::render(current, &mut render_result);
                return Ok(render_result);
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

/// Match user input against registered dispatchers and return the matched dispatcher and remaining arguments.
fn match_user_input<C, G>(
    program: &Program<C, G>,
) -> Result<(&Box<dyn Dispatcher<G>>, Vec<String>), ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = G>,
    G: Display,
{
    let nodes = get_nodes(program);
    let command = format!("{} ", program.args.join(" "));

    // Find all nodes that match the command prefix
    let matching_nodes: Vec<&(String, &Box<dyn Dispatcher<G>>)> = nodes
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
fn render<C: ProgramCollect<Enum = G>, G: Display>(any: AnyOutput<G>) -> RenderResult {
    let mut render_result = RenderResult::default();
    C::render(any, &mut render_result);
    render_result
}

// Get all registered dispatcher names from the program
fn get_nodes<C: ProgramCollect<Enum = G>, G: Display>(
    program: &Program<C, G>,
) -> Vec<(String, &Box<dyn Dispatcher<G>>)> {
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
