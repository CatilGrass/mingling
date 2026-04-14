<h1 align="center">Program</h1>
<p align="center">
    Mingling's Basic Components
</p>

---

## Intro

`Program` is the data structure that holds the state for **Mingling** CLI programs. It manages the user's context and enables type-based dispatch.

`Program` needs to implement the `ProgramCollect` trait, 

> but, you don't have to do this manually — 

The `mingling_macros` crate provides the `gen_program!()` macro, which can auto collect resources defined by the `dispatcher!`, `chain!`, and the `completion!` macro of the `comp` feature.

```rust
// Define Dispatcher
dispatcher!("hello", HelloCommand => HelloEntry);

// Define Renderer
#[renderer]
fn render_hello(_prev: HelloEntry) {
    r_println!("Hello, World!")
}

// Collect all resources here and generate ThisProgram
gen_program!();

// You can also explicitly declare a Program 
// with a different name like this:
// gen_program!(MyProgram);
```

## Adding Setup

You can use the `with_setup` function to add preset [Setup](pages/2-basic/2-setup) to your program, which provide reusable functionality.

For example, you can use the following code to add parsing for global flags like `--confirm` / `--help` / `--quiet` to your program:

```rust
use mingling::{
    macros::gen_program, 
    setup::BasicProgramSetup
};

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    // Add `BasicProgramSetup`
    program.with_setup(BasicProgramSetup);
    program.exec().await;
}

// Generate `ThisProgram`
gen_program!();
```

## Adding Dispatcher

You can use `with_dispatcher` or `with_dispatchers` to add [Dispatchers](pages/2-basic/3-dispatcher) to your program to make it work:

```rust
// Define two Dispatchers using `dispatcher!`
dispatcher!("member.add", 
    AddMemberCommand => AddMemberEntry);
dispatcher!("member.rm", 
    RemoveMemberCommand => RemoveMemberEntry);

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();

    // Register Dispatchers
    program.with_dispatcher(AddMemberCommand);
    program.with_dispatcher(RemoveMemberCommand);

    // Or use `with_dispatchers`
    program.with_dispatchers((
        AddMemberCommand, 
        RemoveMemberCommand
    ));

    program.exec().await;
}
```

## Parsing Global Args

You can extract global arguments before the program runs to control the global state of the `Program`:

```rust
#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();

    let mut output = current_dir().unwrap();

    // Pick the "--quiet" or "-q" flag
    program.global_flag(["--quiet", "-q"], |p| {
        // Disable render output
        p.stdout_setting.render_output = false;
    });

    // Pick the "--output" or "-O" flag, write to output
    program.global_argument(
        ["--output", "-O"], 
        |_, v| output = PathBuf::from(v)
    );

    program.exec().await;
}
```

## 💡 Next Page
> **Basic Component** - Setup [Go](./pages/2-basic/2-setup)
