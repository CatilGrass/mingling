<h1 align="center">General Renderer</h1>
<p align="center">
    Mingling's Features
</p>

---

## Enable Feature

`general_renderer` is a feature provided by **Mingling**. You can enable it in the following way:

```toml
[dependencies]
mingling = { 
    version = "...", 
    features = ["general_renderer"] 
}
```

## Setup

`general_renderer` requires you to implement the `serde::Serialize` trait for **all** structs, so your project needs to include `serde`

```toml
[dependencies]
serde = { 
    version = "1", 
    features = ["derive"] 
}
```

For types wrapped with the `pack!` macro, `serde::Serialize` will be automatically implemented

```rust
pack!(YourInfo = ()); // Auto derive `serde::Serialize`
```

For types using the derive macro `Groupped`, you need to manually implement `serde::Serialize`

```rust
#[derive(Default, Groupped, Serialize)]
struct YourInfo {
    name: String,
    age: i32,
}
```

> [!Tip]
> If there are types that do not implement `serde::Serialize`, compilation will fail.

## Import GeneralRendererSetup

`general_renderer` provides a Setup type called `GeneralRendererSetup`. 

After importing it into your program, 
user inputs like `--json`, `--yaml`, `--toml`, `--ron`, `--json-pretty`, and `--ron-pretty` will be automatically recognized. 

During the **rendering phase**, instead of the **default renderer**, the serialized content will be displayed to the terminal.

```rust
fn main() {
    let mut program = ThisProgram::new();

    // Add General Renderer
    program.with_setup(GeneralRendererSetup);

    // Add Dispatchers
    program.with_dispatchers((
        // Your dispatchers
    ));

    // Execute
    program.exec();
}
```
