//! Example Clap Binding
//!
//! > This example demonstrates how to bind clap_derive to Mingling
//!
//! **Note**:
//! If the `error` parameter of the `dispatcher_clap!` macro is enabled, parameters will be parsed using `try_parse_from`
//! This will cause clap's ColorChoice output to be plain text without ANSI colors
//!
//! Run:
//! ```bash
//! cargo run --manifest-path examples/example-clap-binding/Cargo.toml --quiet -- greet
//! cargo run --manifest-path examples/example-clap-binding/Cargo.toml --quiet -- greet Alice
//! cargo run --manifest-path examples/example-clap-binding/Cargo.toml --quiet -- greet Alice -r 5
//! cargo run --manifest-path examples/example-clap-binding/Cargo.toml --quiet -- greet --help
//! cargo run --manifest-path examples/example-clap-binding/Cargo.toml --quiet -- greet --rppat
//! ```
//!
//! Output:
//! ```plaintext
//! Hello, World!
//! Hello, Alice!
//! Hello, Alice, Alice, Alice, Alice, Alice!
//! Usage: example-clap-binding [OPTIONS] [NAME]
//!
//! Arguments:
//!   [NAME]  [default: World]
//!
//! Options:
//!   -r, --repeat <REPEAT>  [default: 1]
//!   -h, --help             Print help
//!
//! error: unexpected argument '--rppat' found
//!
//!   tip: a similar argument exists: '--repeat'
//!
//! Usage: example-clap-binding --repeat <REPEAT> [NAME]
//!
//! For more information, try '--help'.
//! ```

use mingling::{macros::dispatcher_clap, prelude::*, Groupped};

fn main() {
    let mut program = ThisProgram::new();

    // Set clap help output mode
    program.stdout_setting.clap_help_print_behaviour =
        mingling::ClapHelpPrintBehaviour::PrintDirectly;
    //  mingling::ClapHelpPrintBehaviour::WriteToRenderResult
    //
    // PrintDirectly:
    //   Let Clap print help information directly to stdout
    //
    // WriteToRenderResult:
    //   Capture Clap's help information and write to RenderResult

    program.with_dispatcher(CMDGreet);
    program.exec_and_exit();
}

// Implement Clap Parser, and bind to Dispatcher
//        _______________________________ Default trait, provides fallback on parse failure
//       /         ______________________ clap::Parser, parsing logic implemented by Clap
//       |        /              ________ Implement mingling::Groupped
//       |        |             /           to ensure Mingling can recognize the type
//       vvvvvvv  vvvvvvvvvvvv  vvvvvvvv
#[derive(Default, clap::Parser, Groupped)]
#[dispatcher_clap(
    "greet", CMDGreet,       // Bind EntryGreet to "greet" command
    help = true,             // Generate clap help for EntryGreet
    error = ErrorGreetParsed // Generate and bind error type for parse failure
//  ^^^^^\__ Using `error` intercepts parse failure information into the specified type,
//              which is then rendered by the renderer
)]
pub struct EntryGreet {
    // Positional argument
    #[clap(default_value = "World")]
    name: String,

    // Option argument
    #[arg(short, long, default_value_t = 1)]
    repeat: i32,
}

#[renderer]
fn render_greet(greet: EntryGreet) {
    let name = greet.name;
    let count = greet.repeat.max(0) as usize;

    r_print!("Hello, ");
    for i in 0..count {
        r_print!("{name}");
        if i < count - 1 {
            r_print!(", ");
        }
    }
    r_println!("!");
}

#[renderer]
fn render_greet_parse_failed(err: ErrorGreetParsed) {
    r_println!("{}", err.to_string());
}

gen_program!();
