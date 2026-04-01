# Mìng Lìng - 命令

> [!WARNING]
>
> **Note**: Mingling is still under active development, and its API may change. Feel free to try it out and give us feedback!

`Mingling` is a Rust command-line framework. Its name comes from the Chinese Pinyin for "命令", which means "Command".

## Quick Start

The example below shows how to use `Mingling` to create a simple command-line program:

```rust
use mingling::macros::{dispatcher, gen_program, r_println, renderer};

#[tokio::main]
async fn main() {
    let mut program = DefaultProgram::new();
    program.with_dispatcher(HelloCommand);

    // Execute
    program.exec().await;
}

// Define command: "<bin> hello"
dispatcher!("hello", HelloCommand => HelloEntry);

// Render HelloEntry
#[renderer]
fn render_hello_world(_prev: HelloEntry) {
    r_println!("Hello, World!")
}

// Fallbacks
#[renderer]
fn fallback_dispatcher_not_found(prev: DispatcherNotFound) {
    r_println!("Dispatcher not found for command `{}`", prev.join(", "))
}

#[renderer]
fn fallback_renderer_not_found(prev: RendererNotFound) {
    r_println!("Renderer not found `{}`", *prev)
}

// Collect renderers and chains to generate DefaultProgram
gen_program!();
```

Output:

```
> mycmd hello
Hello, World!
> mycmd hallo
Dispatcher not found for command `hallo`
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

## Next Steps

You can read the following docs to learn more about the `Mingling` framework:

- Check out **[mingling/README.md](mingling/README.md)** to learn about the core library.
- Check out **[mingling_macros/README.md](mingling_macros/README.md)** to learn how to use the macro system.

## Roadmap

- [ ] Parallel Chains ( Parallel\<AnyOutput, AnyOutput\> )
- [ ] General Renderers ( Json, Yaml, Toml, Ron )
- [ ] Completion
- [ ] ...

## License

This project is licensed under the MIT License. 

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.
