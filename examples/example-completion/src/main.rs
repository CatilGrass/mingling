//! `Mingling` Example - Completion
//!
//! # How to Deploy
//! 1. Enable the `comp` feature
//! ```toml
//! mingling = { version = "0.1.5", features = [
//!     "comp",  // Enable this feature
//!     "parser"
//! ] }
//! ```
//!
//! 2. Write `build.rs` to generate completion scripts at compile time
//! ```ignore
//! use mingling::build::{build_comp_scripts, build_comp_scripts_with_bin_name};
//! fn main() {
//!     // Generate completion scripts for the current program
//!     build_comp_scripts().unwrap();
//!
//!     // Or specify a specific name
//!     // build_comp_scripts_with_bin_name("your_bin").unwrap();
//! }
//! ```
//!
//! 3. Write `main.rs`, adding completion logic for your command entry point
//! 4. Execute `cargo install --path ./`, then run the corresponding completion script in your shell

use mingling::{
    AnyOutput, Groupped, ShellContext, Suggest,
    macros::{chain, completion, dispatcher, gen_program, r_println, renderer, suggest},
    marker::NextProcess,
    parser::{Pickable, Picker},
};

// Define dispatcher `FruitCommand`, directing subcommand "fruit" to `FruitEntry`
dispatcher!("fruit", FruitCommand => FruitEntry);

#[completion(FruitEntry)]
fn comp_fruit_command(ctx: &ShellContext) -> Suggest {
    // When the user is filling "--name" for the first time
    if ctx.filling_argument_first("--name") {
        return suggest!();
    }
    // When the user is filling "--type" for the first time
    if ctx.filling_argument_first("--type") {
        return suggest! {
            "apple", "banana"
        };
    }
    // When the user is typing an argument
    if ctx.typing_argument() {
        return suggest! {
            "--name": "Fruit name",
            "--type": "Fruit type"
        }
        // Strip already typed arguments
        .strip_typed_argument(ctx);
    }

    // Return empty suggestion, indicating Shell should not perform any completion logic
    return suggest!();
}

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CompletionDispatcher); // Add completion dispatcher
    program.with_dispatcher(FruitCommand);
    program.exec().await;
}

#[derive(Groupped)]
struct FruitInfo {
    name: String,
    fruit_type: FruitType,
}

#[derive(Default, Debug)]
enum FruitType {
    #[default]
    Apple,
    Banana,
    Other(String),
}

impl Pickable for FruitType {
    type Output = FruitType;

    fn pick(args: &mut mingling::parser::Argument, flag: mingling::Flag) -> Option<Self::Output> {
        let name = args.pick_argument(flag);
        match name {
            Some(name) => match name.as_str() {
                "apple" => Some(FruitType::Apple),
                "banana" => Some(FruitType::Banana),
                other => Some(FruitType::Other(other.to_string())),
            },
            None => None,
        }
    }
}

#[chain]
async fn parse_fruit_info(prev: FruitEntry) -> NextProcess {
    let picker = Picker::<ThisProgram>::from(prev.inner);
    let (fruit_name, fruit_type) = picker.pick("--name").pick("--type").unpack_directly();
    let info = FruitInfo {
        name: fruit_name,
        fruit_type,
    };
    AnyOutput::new(info).route_renderer()
}

#[renderer]
fn render_fruit(prev: FruitInfo) {
    if let FruitType::Other(other) = prev.fruit_type {
        r_println!("Fruit name: {}, Type: {:?} (Unknown)", prev.name, other);
    } else {
        r_println!("Fruit name: {}, Type: {:?}", prev.name, prev.fruit_type);
    }
}

gen_program!();
