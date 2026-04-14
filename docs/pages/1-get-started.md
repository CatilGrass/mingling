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
mingling = "0.1.5"
```

> **Mingling** is an **async program**, so please use `async-std`, `tokio`, or another async runtime.

2. This article assumes you are using the `tokio` async runtime. Add the following to your `Cargo.toml`:
```toml
tokio = { 
    version = "1", 
    features = [
        "macros", 
        "rt", 
        "rt-multi-thread"
    ] 
}
```

3. Write the basic code in your `main.rs` or other program entry point.
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

4. Install your command-line program and run it.
```bash
cargo install --path ./
your_bin hello
```
Result:
```bash
Hello, World!
```

## 💡 Next Steps
> **Mingling**'s basic components [Go](./pages/2-basic)
