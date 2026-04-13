// Auto generated

/// `Mingling` Example - Basic
///
///  # How to Run
///  ```bash
///  cargo run --manifest-path ./examples/example-basic/Cargo.toml -- hello World
///  ```
///
/// Cargo.toml
/// ```ignore
/// [package]
/// name = "example-basic"
/// version = "0.0.1"
/// edition = "2024"
///
/// [dependencies]
/// mingling = { path = "../../mingling" }
/// tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros"] }
/// ```
///
/// main.rs
/// ```ignore
/// use mingling::{
///     macros::{chain, dispatcher, gen_program, pack, r_println, renderer},
///     marker::NextProcess,
/// };
///
/// // Define dispatcher `HelloCommand`, directing subcommand "hello" to `HelloEntry`
/// dispatcher!("hello", HelloCommand => HelloEntry);
///
/// #[tokio::main]
/// async fn main() {
///     // Create program
///     let mut program = ThisProgram::new();
///
///     // Add dispatcher `HelloCommand`
///     program.with_dispatcher(HelloCommand);
///
///     // Run program
///     program.exec().await;
/// }
///
/// // Register wrapper type `Hello`, setting inner to `String`
/// pack!(Hello = String);
///
/// // Register chain to `ThisProgram`, handling logic from `HelloEntry`
/// #[chain]
/// async fn parse_name(prev: HelloEntry) -> NextProcess {
///     // Extract string from `HelloEntry` as argument
///     let name = prev.first().cloned().unwrap_or_else(|| "World".to_string());
///
///     // Build `Hello` type and route to renderer
///     Hello::new(name).to_render()
/// }
///
/// // Register renderer to `ThisProgram`, handling rendering of `Hello`
/// #[renderer]
/// fn render_hello_who(prev: Hello) {
///     // Print message
///     r_println!("Hello, {}!", *prev);
///
///     // Program ends here
/// }
///
/// // Generate program, default is `ThisProgram`
/// gen_program!();
/// ```
pub mod example_basic {}
/// `Mingling` Example - Completion
///
///  # How to Deploy
///  1. Enable the `comp` feature
///  ```toml
///  mingling = { version = "0.1.5", features = [
///      "comp",  // Enable this feature
///      "parser"
///  ] }
///  ```
///
///  2. Write `build.rs` to generate completion scripts at compile time
///  ```ignore
///  use mingling::build::{build_comp_scripts, build_comp_scripts_with_bin_name};
///  fn main() {
///      // Generate completion scripts for the current program, using the Cargo package name as the binary filename
///      build_comp_scripts(env!("CARGO_PKG_NAME")).unwrap();
///
///      // Or, explicitly specify the binary filename
///      // build_comp_scripts("your_bin").unwrap();
///  }
///  ```
///
///  3. Write `main.rs`, adding completion logic for your command entry point
///  4. Execute `cargo install --path ./`, then run the corresponding completion script in your shell
///
/// Cargo.toml
/// ```ignore
/// [package]
/// name = "example-completion"
/// version = "0.0.1"
/// edition = "2024"
///
/// [dependencies]
/// mingling = { path = "../../mingling", features = ["comp", "parser"] }
/// tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros"] }
/// ```
///
/// main.rs
/// ```ignore
/// use mingling::{
///     AnyOutput, Groupped, ShellContext, Suggest,
///     macros::{chain, completion, dispatcher, gen_program, r_println, renderer, suggest},
///     marker::NextProcess,
///     parser::{Pickable, Picker},
/// };
///
/// // Define dispatcher `FruitCommand`, directing subcommand "fruit" to `FruitEntry`
/// dispatcher!("fruit", FruitCommand => FruitEntry);
///
/// #[completion(FruitEntry)]
/// fn comp_fruit_command(ctx: &ShellContext) -> Suggest {
///     // When the user is filling "--name" for the first time
///     if ctx.filling_argument_first("--name") {
///         return suggest!();
///     }
///     // When the user is filling "--type" for the first time
///     if ctx.filling_argument_first("--type") {
///         return suggest! {
///             "apple", "banana"
///         };
///     }
///     // When the user is typing an argument
///     if ctx.typing_argument() {
///         return suggest! {
///             "--name": "Fruit name",
///             "--type": "Fruit type"
///         }
///         // Strip already typed arguments
///         .strip_typed_argument(ctx);
///     }
///
///     // Return empty suggestion, indicating Shell should not perform any completion logic
///     return suggest!();
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut program = ThisProgram::new();
///     program.with_dispatcher(CompletionDispatcher); // Add completion dispatcher
///     program.with_dispatcher(FruitCommand);
///     program.exec().await;
/// }
///
/// #[derive(Groupped)]
/// struct FruitInfo {
///     name: String,
///     fruit_type: FruitType,
/// }
///
/// #[derive(Default, Debug)]
/// enum FruitType {
///     #[default]
///     Apple,
///     Banana,
///     Other(String),
/// }
///
/// impl Pickable for FruitType {
///     type Output = FruitType;
///
///     fn pick(args: &mut mingling::parser::Argument, flag: mingling::Flag) -> Option<Self::Output> {
///         let name = args.pick_argument(flag);
///         match name {
///             Some(name) => match name.as_str() {
///                 "apple" => Some(FruitType::Apple),
///                 "banana" => Some(FruitType::Banana),
///                 other => Some(FruitType::Other(other.to_string())),
///             },
///             None => None,
///         }
///     }
/// }
///
/// #[chain]
/// async fn parse_fruit_info(prev: FruitEntry) -> NextProcess {
///     let picker = Picker::<ThisProgram>::from(prev.inner);
///     let (fruit_name, fruit_type) = picker.pick("--name").pick("--type").unpack_directly();
///     let info = FruitInfo {
///         name: fruit_name,
///         fruit_type,
///     };
///     AnyOutput::new(info).route_renderer()
/// }
///
/// #[renderer]
/// fn render_fruit(prev: FruitInfo) {
///     if let FruitType::Other(other) = prev.fruit_type {
///         r_println!("Fruit name: {}, Type: {:?} (Unknown)", prev.name, other);
///     } else {
///         r_println!("Fruit name: {}, Type: {:?}", prev.name, prev.fruit_type);
///     }
/// }
///
/// gen_program!();
/// ```
pub mod example_completion {}
/// `Mingling` Example - General Renderer
///
///  ## Step1 - Enable Feature
///  Enable the `general_renderer` feature for mingling in `Cargo.toml`
///  ```toml
///  [dependencies]
///  mingling = { version = "...", features = ["general_renderer", "parser"] }
///  ```
///
///  ## Step2 - Add Dependencies
///  Add `serde` dependency to `Cargo.toml` for serialization support
///  ```toml
///  [dependencies]
///  serde = { version = "1", features = ["derive"] }
///  ```
///
///  ## Step3 - Write Code
///  Write the following content into `main.rs`
///
///  ## Step4 - Build and Run
///  ```bash
///  cargo run --manifest-path ./examples/example-general-renderer/Cargo.toml -- render Bob 22
///  cargo run --manifest-path ./examples/example-general-renderer/Cargo.toml -- render Bob 22 --json
///  cargo run --manifest-path ./examples/example-general-renderer/Cargo.toml -- render Bob 22 --yaml
///  ```
///
///  Will print:
///  ```plain
///  Bob is 22 years old
///  {"member_name":"Bob","member_age":22}
///  member_name: Bob
///  member_age: 22
///  ```
///
/// Cargo.toml
/// ```ignore
/// [package]
/// name = "example-general-renderer"
/// version = "0.0.1"
/// edition = "2024"
///
/// [dependencies]
/// mingling = { path = "../../mingling", features = [
///     "parser",
///     "general_renderer",
/// ] }
/// serde = { version = "1", features = ["derive"] }
/// tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros"] }
/// ```
///
/// main.rs
/// ```ignore
/// use mingling::{
///     AnyOutput, Groupped,
///     macros::{chain, dispatcher, gen_program, r_println, renderer},
///     marker::NextProcess,
///     parser::Picker,
///     setup::GeneralRendererSetup,
/// };
/// use serde::Serialize;
///
/// dispatcher!("render", RenderCommand => RenderCommandEntry);
///
/// #[tokio::main]
/// async fn main() {
///     let mut program = ThisProgram::new();
///     // Add `GeneralRendererSetup` to receive user input `--json` `--yaml` parameters
///     program.with_setup(GeneralRendererSetup);
///     program.with_dispatcher(RenderCommand);
///     program.exec().await;
/// }
///
/// // Manually implement Info struct
/// #[derive(Serialize, Groupped)]
/// struct Info {
///     #[serde(rename = "member_name")]
///     name: String,
///     #[serde(rename = "member_age")]
///     age: i32,
/// }
///
/// #[chain]
/// async fn parse_render(prev: RenderCommandEntry) -> NextProcess {
///     let (name, age) = Picker::<AnyOutput<ThisProgram>>::new(prev.inner)
///         .pick::<String>(())
///         .pick::<i32>(())
///         .unpack_directly();
///     AnyOutput::new(Info { name, age }).route_renderer()
/// }
///
/// // Implement default renderer for when general_renderer is not specified
/// #[renderer]
/// fn render_info(prev: Info) {
///     r_println!("{} is {} years old", prev.name, prev.age);
/// }
///
/// gen_program!();
/// ```
pub mod example_general_renderer {}
/// `Mingling` Example - Picker
///
///  ## Step1 - Enable Feature
///  Enable the `parser` feature for mingling in `Cargo.toml`
///  ```toml
///  [dependencies]
///  mingling = { version = "...", features = ["parser"] }
///  ```
///
///  ## Step2 - Write Code
///  Write the following content into `main.rs`
///
///  ## Step3 - Build and Run
///  ```bash
///  cargo run --manifest-path ./examples/example-picker/Cargo.toml -- pick Bob
///  cargo run --manifest-path ./examples/example-picker/Cargo.toml -- pick Bob --age -15
///  cargo run --manifest-path ./examples/example-picker/Cargo.toml -- pick --age 99
///  ```
///
/// Cargo.toml
/// ```ignore
/// [package]
/// name = "example-picker"
/// version = "0.0.1"
/// edition = "2024"
///
/// [dependencies]
/// mingling = { path = "../../mingling", features = ["parser"] }
/// tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros"] }
/// ```
///
/// main.rs
/// ```ignore
/// use mingling::{
///     AnyOutput,
///     macros::{chain, dispatcher, gen_program, pack, r_println, renderer},
///     marker::NextProcess,
///     parser::Picker,
/// };
///
/// dispatcher!("pick", PickCommand => PickEntry);
///
/// #[tokio::main]
/// async fn main() {
///     let mut program = ThisProgram::new();
///     program.with_dispatcher(PickCommand);
///     program.exec().await;
/// }
///
/// pack!(NoNameProvided = ());
/// pack!(ParsedPickInput = (i32, String));
///
/// #[chain]
/// async fn parse(prev: PickEntry) -> NextProcess {
///     // Extract arguments from `PickEntry`'s inner and create a `Picker`
///     let picker = Picker::new(prev.inner);
///     let picked = picker
///         // First extract the named argument
///         .pick_or("--age", 20)
///         .after(|n: i32| n.clamp(0, 100))
///         // Then sequentially extract the remaining arguments
///         .pick_or_route((), AnyOutput::new(NoNameProvided::default()))
///         .unpack();
///
///     match picked {
///         Ok(value) => ParsedPickInput::new(value).to_render(),
///         Err(e) => e.route_renderer(),
///     }
/// }
///
/// #[renderer]
/// fn render_parsed_pick_input(prev: ParsedPickInput) {
///     let (age, name) = prev.inner;
///     r_println!("Picked: name = {}, age = {}", name, age);
/// }
///
/// #[renderer]
/// fn render_no_name_input(_prev: NoNameProvided) {
///     r_println!("No name provided.");
/// }
///
/// gen_program!();
/// ```
pub mod example_picker {}
