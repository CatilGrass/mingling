//! Example Hook
//!
//! > This example demonstrates how to use Mingling's hook system to obtain debugging information during program execution
//!
//! Run:
//! ```base
//! cargo run --manifest-path examples/example-hook/Cargo.toml --quiet -- greet Alice
//! ```
//!
//! Output:
//! ```plaintext
//! [DEBUG] Program is begin
//! [DEBUG] Pre dispatch: ["greet", "Alice"]
//! [DEBUG] Post dispatch: EntryGreet
//! [DEBUG] Pre chain: EntryGreet
//! [DEBUG] Post chain: ResultName
//! [DEBUG] Pre render: ResultName
//! [DEBUG] Post render
//! [DEBUG] Program end
//! Hello, Alice!
//! ```

use mingling::{hook::ProgramHook, prelude::*};

dispatcher!("greet", CMDGreet => EntryGreet);

fn main() {
    let mut program = ThisProgram::new();

    // --------- IMPORTANT ---------
    program.with_hook(
        ProgramHook::<ThisProgram>::empty()
            .on_begin(|| println!("[DEBUG] Program is begin"))
            .on_pre_dispatch(|args| println!("[DEBUG] Pre dispatch: {args:?}"))
            .on_post_dispatch(|c: &_| println!("[DEBUG] Post dispatch: {c:?}"))
            .on_pre_chain(|c: &_, _| {
                println!("[DEBUG] Pre chain: {c}");
            })
            .on_post_chain(|any_output| println!("[DEBUG] Post chain: {}", any_output.member_id))
            .on_finish(|| {
                println!("[DEBUG] Loop end");
                0 // Override exit code
            })
            .on_pre_render(|c: &_, _| println!("[DEBUG] Pre render: {c}"))
            .on_post_render(|_| println!("[DEBUG] Post render")),
    );
    // --------- IMPORTANT ---------

    program.with_dispatcher(CMDGreet);
    program.exec_and_exit();
}

pack!(ResultName = String);

#[chain]
fn handle_greet(args: EntryGreet) -> Next {
    let name: ResultName = args
        .inner
        .first()
        .cloned()
        .unwrap_or_else(|| "World".to_string())
        .into();
    name
}

/// Renders the greeting message with the provided name.
#[renderer]
fn render_name(name: ResultName) {
    r_println!("Hello, {}!", *name);
}

gen_program!();
