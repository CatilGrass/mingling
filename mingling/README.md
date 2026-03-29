# Mìng Lìng - 命令

> [!WARNING]
>
> **Note**: Mingling is still under active development, and its API may change. Feel free to try it out and give us feedback!

`Mingling` is a Rust command-line framework. Its name comes from the Chinese Pinyin for "命令", which means "Command".

## Quick Start

The example below shows how to use `Mingling` to create a simple command-line program:

```rust
use mingling::{
    hint::NoDispatcherFound,
    macros::{dispatcher, program, r_println, renderer},
};

#[tokio::main]
async fn main() {
    let mut program = MyProgram::new();
    program.with_dispatcher(HelloCommand);
    program.exec().await;
}

dispatcher!("hello", HelloCommand => HelloEntry);

#[renderer]
pub fn render_hello(_prev: HelloEntry) {
    r_println!("Hello, World!")
}

#[renderer]
pub fn render_no_dispatcher_found(prev: NoDispatcherFound) {
    r_println!("Subcommand not found: '{}'", prev.args.join(", "))
}

program!(MyProgram);
```

Output:

```
> mycmd hello
Hello, World!
> mycmd hallo
Subcommand not found: 'mycmd hallo'
```

## Core Concepts

Mingling abstracts command execution into the following parts:

1. **Dispatcher** - Routes user input to a specific renderer or chain based on the command node name.
2. **Chain** - Transforms the incoming type into another type, passing it to the next chain or renderer.
3. **Renderer** - Stops the chain and prints the currently processed type to the terminal.
4. **Program** - Manages the lifecycle and configuration of the entire CLI application.

## Project Structure

The Mingling project consists of two main parts:

- **[mingling/](mingling/)** - The core runtime library, containing type definitions, error handling, and basic functionality.
- **[mingling_macros/](mingling_macros/)** - The procedural macro library, providing declarative macros to simplify development.

## License

This project is licensed under the MIT License. 

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.
