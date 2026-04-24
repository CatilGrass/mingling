<h1 align="center">Completion</h1>
<p align="center">
    Mingling's Features
</p>

---

## Enable Feature

`comp` is the command-line completion feature provided by **Mingling**. Its approach is not static completion but rather dynamic completion by invoking your program itself.

Enable this feature as follows:

```toml
[dependencies]
mingling = { 
    version = "...", 
    features = ["comp"] 
}
```

## Setup

Once `comp` is enabled, `gen_program!` will automatically generate a `CompletionDispatcher`, which is a command with the node `__comp`: the completion script will call this subcommand.

Add this [Dispatcher](pages/2-basic/3-dispatcher) to your [Program](pages/2-basic/1-program):

```rust
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CompletionDispatcher);
    program.exec();
}
```

## Usage

You can use the `completion!` macro to bind completion logic to your command entry point. The syntax is as follows:

```rust
// Define Dispatcher
dispatcher!("test-comp", 
    TestCompletionCommand => TestCompletionEntry
);

// Establish completion logic, bound to `TestCompletionEntry`
#[completion(TestCompletionEntry)]
fn comp_test_comp_cmd(_ctx: &ShellContext) -> Suggest {
    suggest!()
}
```

You can obtain the context passed by the shell via `ShellContext` and return the generated suggestions:

```rust
#[completion(TestCompletionEntry)]
fn comp_test_comp_cmd(ctx: &ShellContext) -> Suggest {
    if ctx.current_word.starts_with("-") {
        // Comp flags
        return suggest!(
            "--name": "Names",
            "--age": "Age"
        );
    }

    if ctx.previous_word == "--name" {
        return suggest!("Bob", "Alice"); // Comp names
    }

    if ctx.previous_word == "--age" {
        return suggest!(); // If typing age, suggest nothing
    }

    suggest!() // Comp nothing
}
```

> 🎬 Logic
>
> When the user inputs `bin test-<TAB>`, it completes to `bin test-comp`.
>
> When the user inputs `bin test-comp -<TAB>`, it suggests `--age` / `--name`.
>
> When the user inputs `bin test-comp --name <TAB>`, it suggests `Bob` / `Alice`.
>
> In other cases, no suggestions are generated.

## Generate Completion Script

Any shell requires registering a relevant completion script to enable your command's completion capability. However, **Mingling** provides a related build script:

Please add the following to `build-dependencies` in your `Cargo.toml`:

```toml
[build-dependencies]
mingling = { version = "...", features = ["comp"] }
```

Next, call the following logic in your project's `build.rs`:

```rust
use mingling::build::build_comp_scripts;

fn main() {
    // Generate completion scripts for the current program
    // build_comp_scripts().unwrap();

    // Or specify a specific name
    build_comp_scripts("your_cmd").unwrap();
}
```

`build_comp_scripts` will generate the corresponding completion scripts based on your platform and output them to the `target` directory.

> [!Note]
> The completion script does not contain the actual completion logic; 
>
> it is just a thin invocation layer.
