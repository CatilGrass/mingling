<h1 align="center">Setup</h1>
<p align="center">
    Mingling's Basic Components
</p>

---

## Intro

`Setup` is used to organize and package the initialization process of a `Program`, making the project easier to manage. 

## Usage

It is defined as follows:

```rust
struct MySetup;
impl ProgramSetup<ThisProgram> 
    for MySetup 
{
    fn setup(
        &mut self, 
        program: &mut Program<ThisProgram>
    ) {
        // Your setup logic
    }
}
```

For example:

```rust
use std::{env::current_dir, path::PathBuf};

use mingling::{
    Program,
    macros::{dispatcher, gen_program, renderer},
    setup::ProgramSetup,
};

// Global state
static OUTPUT_PATH: std::sync::OnceLock<PathBuf> 
    = std::sync::OnceLock::new();

fn main() {
    let mut program = ThisProgram::new();
    program.with_setup(MySetup);
    program.exec();
}

// Define two Dispatchers using `dispatcher!`
dispatcher!("member.add",
    AddMemberCommand => AddMemberEntry);
dispatcher!("member.rm",
    RemoveMemberCommand => RemoveMemberEntry);

struct MySetup;
impl ProgramSetup<ThisProgram, ThisProgram> for MySetup {
    fn setup(
        &mut self, program: &mut Program<ThisProgram, ThisProgram>
    ) {
        // Register Dispatchers
        program.with_dispatcher(AddMemberCommand);
        program.with_dispatcher(RemoveMemberCommand);

        // Initialize global output once
        OUTPUT_PATH.get_or_init(|| current_dir().unwrap());

        // Pick the "--quiet" or "-q" flag
        program.global_flag(["--quiet", "-q"], |p| {
            // Disable render output
            p.stdout_setting.render_output = false;
        });

        // Pick the "--output" or "-O" flag, write to output
        program.global_argument(["--output", "-O"], |_, v| {
            let _ = OUTPUT_PATH.set(PathBuf::from(v));
        });
    }
}

gen_program!();
```

## Simplified Syntax

If you find the above declaration method too **verbose**, you can use the `program_setup!` macro to simplify it. The format is:

```rust
#[program_setup]
fn my_setup(
    program: &mut Program<ThisProgram>
) {
    // Your setup logic
}
```

For example:

```rust
use std::{env::current_dir, path::PathBuf};

use mingling::{
    Program,
    macros::{
        dispatcher, 
        gen_program, 
        program_setup, 
        renderer
    },
};

static OUTPUT_PATH: std::sync::OnceLock<PathBuf> 
    = std::sync::OnceLock::new();

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_setup(MySetup);
    program.exec().await;
}

dispatcher!("member.add",
    AddMemberCommand => AddMemberEntry);
dispatcher!("member.rm",
    RemoveMemberCommand => RemoveMemberEntry);

#[program_setup]
fn my_setup(
    program: &mut Program<ThisProgram>
) {
    program.with_dispatcher(AddMemberCommand);
    program.with_dispatcher(RemoveMemberCommand);

    OUTPUT_PATH.get_or_init(|| current_dir().unwrap());

    program.global_flag(["--quiet", "-q"], |p| {
        p.stdout_setting.render_output = false;
    });

    program.global_argument(["--output", "-O"], |_, v| {
        let _ = OUTPUT_PATH.set(PathBuf::from(v));
    });
}

gen_program!();
```

## 💡 Next Page
> **Basic Component** - Dispatcher [Go](./pages/2-basic/3-dispatcher)
