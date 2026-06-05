<p align="center">
    <a href="https://github.com/catilgrass/mingling">
        <img alt="Mingling" src="https://github.com/catilgrass/mingling/raw/main/docs/res/pixel_icon_o_1024.png" width="30%">
    </a>
</p>
<h1 align="center">Mìng Lìng - 命令</h1>

<p align="center">
    Macro magician in your CLI.
</p>
<p align="center">
	<img src="https://img.shields.io/github/stars/catilgrass/mingling?style=flat"> 
	<a href="https://crates.io/crates/mingling">
	    <img src="https://img.shields.io/crates/v/mingling?style=flat">
	</a>
	<a href="https://docs.rs/mingling/latest/mingling/">
	    <img src="https://img.shields.io/docsrs/mingling?style=flat">
	</a>	
	<a href="https://catilgrass.github.io/mingling/">
	    <img src="https://img.shields.io/badge/helpdoc-rewriting-yellow?style=flat">
	</a>
</p>


> [!WARNING]
>
> **Note**: Mingling is still under active development, and its API may change. Feel free to try it out and give us feedback!
> **Hint**: This note will be removed in version `0.5.0`

<h1 align="center">
  🤔 What is Mingling? 🤔 
</h1>

[`Mingling`](https://github.com/catilgrass/mingling) is a **proc-macro and type system-based** Rust CLI framework, suitable for developing complex command-line programs with numerous subcommands.

> **BTW:** Its name comes from the Chinese Pinyin **"Mìng Lìng"**, meaning **"Command"**. 😄

### Mingling's Core Capabilities

1. **Separation of Concerns, Clear Logic**: Mingling decouples logic by responsibility, helping you organize your CLI program more clearly.
See example: [Example](https://github.com/catilgrass/mingling/blob/main/examples/example-basic/src/main.rs)
2. **"All Logic is Functions"**: Execution logic, rendering logic, completion logic, help logic — everything is a function. Just attach the corresponding attribute macro to bind them to your program.
3. **Fully Dynamic Completion System**: With the `comp` feature, you can flexibly implement dynamic completion logic for any subcommand.
See examples: [Example](https://github.com/catilgrass/mingling/blob/main/examples/example-completion/src/main.rs)
4. **Lightning-Fast Subcommand Dispatch**: With the `dispatch_tree` feature, Mingling hardens the subcommand structure into a prefix tree at **compile time**, enabling blazing-fast subcommand lookup.
See examples: [Example](https://github.com/catilgrass/mingling/blob/main/examples/example-dispatch-tree/src/main.rs)
5. **Lightweight Dependencies, On-Demand Importing**: Minimal core dependencies keep builds fast; enhanced features are imported on demand through fine-grained feature flags.
6. **Structured Output**: Enabling the `general_renderer` feature adds support for flags like `--json` and `--yaml`, providing structured output capabilities.
See examples: [Example](https://github.com/catilgrass/mingling/blob/main/examples/example-general-renderer/src/main.rs)


### What does Mingling look like?

Here is a basic project written using **Mingling**:
- When the user types `greet`, the program outputs `Hello, World!`
- When the user types `greet Alice`, the program outputs `Hello, Alice!`

```rust
use mingling::prelude::*;

dispatcher!("greet", CMDGreet => EntryGreet);

fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDGreet);
    program.exec_and_exit();
}

pack!(ResultGreeting = String);

#[chain]
fn handle_greet(args: EntryGreet) -> Next {
    let greeting = args.pick_or::<String>((), "World").unpack();
    ResultGreeting::new(greeting)
}

#[renderer]
fn render_greeting(greeting: ResultGreeting) {
    r_println!("Hello, {}!", *greeting);
}

gen_program!();
```

<h1 align="center">
    🚀 How to get started? 🚀
</h1>

There are multiple ways to import **Mingling**:

1. From [crates.io](https://crates.io/crates/mingling):
```toml
[dependencies.mingling]
version = "0.1.9"
features = []
```

2. From [GitHub](https://github.com/catilgrass/mingling):
```toml
[dependencies.mingling]
git = "https://github.com/catilgrass/mingling"
branch = "main"
features = []
```

3. Alternatively, you can quickly scaffold a new project from the [Mingling-Template](https://github.com/catilgrass/mingling-template) using:
```bash
cargo generate --git catilgrass/mingling-template
```

<h1 align="center">
    💡 How to learn? 💡
</h1>

You can read the following docs to learn more about the `Mingling` framework:

- 💡 Check out **[Mingling Helpdoc](https://catilgrass.github.io/mingling/)** to learn the basics.
- 💡 Check out **[Examples](https://docs.rs/mingling/latest/mingling/_mingling_examples/index.html)** to learn about the core library.
- 💡 Check out **[docs.rs](https://docs.rs/mingling/latest/mingling/)** to learn how to use the macro system and explore the full API.

<h1 align="center">
    🗺️ Roadmap 🗺️
</h1>

- [ ] Milestone.1 "MVP"
  - [x] \[[0.1.4](https://docs.rs/mingling/0.1.4/mingling/)\] \[`core`\] \[`general_renderer`\] **Mingling** can render data into serializable formats via `--json` and `--yaml` flags
  - [x] \[[0.1.5](https://docs.rs/mingling/0.1.5/mingling/)\] \[`core`\] \[`comp`\] **Mingling** can dynamically invoke itself to provide completions for shells like `bash`, `zsh`, `fish`, and `pwsh`
  - [x] \[[0.1.6](https://docs.rs/mingling/0.1.6/mingling/)\] \[`core`\] \[`comp`\] **Mingling** can gather more context for smarter completions
  - [x] \[[0.1.7](https://docs.rs/mingling/0.1.7/mingling/)\] \[`clap`\] Provides a **Clap** compatibility layer, allowing **Mingling** to reuse its powerful parsing capabilities
  - [x] \[[0.1.7](https://docs.rs/mingling/0.1.7/mingling/)\] \[`core`\] **Mingling** can intercept `-h` or `--help` flags to display custom help text for each subcommand
  - [x] \[[0.1.7](https://docs.rs/mingling/0.1.7/mingling/)\] \[`mling`\] Provides a basic scaffolding tool (`mling`) for rapid development and debugging
  - [x] \[[0.1.8](https://docs.rs/mingling/0.1.8/mingling/)\] \[`core`\] \[`dispatch_tree`\] Converts the subcommand list into a prefix tree to improve command matching speed
  - [X] \[[0.1.9](https://docs.rs/mingling/0.1.9/mingling/)\] \[`core`\] \[`dev_toolkits`\] Provides debugging interfaces for developers to capture invocation information when issues arise (`InvokeStackDisplay`) (indirectly implemented via `ProgramHook`)
  - [X] \[[0.1.9](https://docs.rs/mingling/0.1.9/mingling/)\] \[`core`\] \[`repl`\] Provides REPL capability (`program.exec_repl();`)
  - [ ] \[**0.2.0**\] Complete documentation, tests, and examples

- [ ] Milestone.2 "More Comfortable Dev and User Experience"
  - [ ] \[**0.2.1**\] \[`macros`\] `r_println!` in `#[chain]` support.
  - [ ] \[**0.2.5**\] \[`mling`\] Helpdoc Maker
  - [ ] \[**0.2.8**\] \[`picker`\] A more efficient and intelligent argument parser

- [ ] Milestone.3 "Unplanned"
  - [ ] ...

<h1 align="center">
    🚫 Unplanned Features 🚫
</h1>

While Mingling has several common CLI features that are **NOT PLANNED** to be directly included in the framework.
This is because the Rust ecosystem already has excellent and mature crates to handle these issues, and Mingling's design is intended to be used in combination with them.

- **Colored Output**: To add color and styles (bold, italic, etc.) to terminal output, consider using crates like [`colored`](https://crates.io/crates/colored) or [`owo-colors`](https://crates.io/crates/owo-colors). You can integrate their types directly into your renderers.
- **I18n**: To translate your CLI application, the [`rust-i18n`](https://crates.io/crates/rust-i18n) crate provides a powerful internationalization solution that you can use in your command logic and renderers.
- **Progress Bars**: To display progress indicators, the [`indicatif`](https://crates.io/crates/indicatif) crate is the standard choice.
- **TUI**: To build full-screen interactive terminal applications, it is recommended to use a framework like [`ratatui`](https://crates.io/crates/ratatui) (formerly `tui-rs`).

<h1 align="center">
    📄 Open Source License 📄
</h1>

This project is licensed under the MIT License. 

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.
