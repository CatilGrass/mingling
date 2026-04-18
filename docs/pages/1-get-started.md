# Get Started
This article explains how to quickly create your first **Mingling** command-line program.

## Quick Start
1. Add `mingling` to your Rust project.
```bash
cargo add mingling
```
Or add the following to your `Cargo.toml`:
```toml
[dependencies]
mingling = "0.1.6"
```

2. Write the basic code in your `main.rs` or other program entry point.
```rust
use mingling::macros::{dispatcher, gen_program, r_println, renderer};
 
#[tokio::main]
async fn main() {
    // Create ThisProgram
    let mut program = ThisProgram::new();
 
    // Import the dispatcher `HelloCommand`
    program.with_dispatcher(HelloCommand);
 
    // Run the program
    program.exec().await;
}
 
// Define the dispatcher `HelloCommand`, which routes the "hello" subcommand to `HelloEntry`
dispatcher!("hello", HelloCommand => HelloEntry);
 
// Define the renderer, which receives `HelloEntry` and renders the content
#[renderer]
fn render_hello(_prev: HelloEntry) {
    r_println!("Hello, World!")
}
 
// Create ThisProgram at the end of the code
gen_program!();
```

3. Install your command-line program and run it.
```bash
cargo install --path ./
your_bin hello
```
Result:
```bash
Hello, World!
```

## About Async Runtime

**Mingling** supports **async runtime**, you can enable the `async` feature to activate it.

After enabling it, **Mingling** will have the following changes:

- The `Chain` trait and `chain!` macro will require you to use **async functions**
- `Program::exec` will become an async function
- The `gen_program!` macro will generate async functions

**Mingling** does not depend on any specific asynchronous runtime internally, which means you can freely choose a suitable asynchronous runtime for your program (such as `async-std`, `tokio`)

## 💡 Next Steps
> **Mingling**'s basic components [Go](./pages/2-basic)
