<h1 align="center">Renderer</h1>
<p align="center">
    Mingling's Basic Components
</p>

---

## Intro

`Renderer` is similar to [Chain](pages/2-basic/4-chain) in that they both handle processing for a specific type. The difference is: [Chain](pages/2-basic/4-chain) transforms the type, while `Renderer` terminates the program and prints the information of that type to the terminal.

A type can be processed by both [Chain](pages/2-basic/4-chain) and `Renderer`. If the type is `route_chain`ed, the system will search for a [Chain](pages/2-basic/4-chain) capable of handling that type. If none is found, it will automatically be routed to the `Renderer` to print the result of that type.

The following example demonstrates how to handle rendering logic:

```rust
dispatcher!("hello",
    HelloCommand => HelloEntry);

pack!(ParsedHello = String);

// It's the Chain defined in the Dispatcher chapter
#[chain]
async fn parse_hello(prev: HelloEntry) -> NextProcess {
    let args = &*prev;
    let first = args
        .first()
        .cloned()
        .unwrap_or_else(|| 
            "World".to_string()
        );

    // Distribute the type to the Renderer
    ParsedHello::new(first).to_render()
}

// Define the renderer to 
//   handle rendering of ParsedHello
#[renderer]
fn render_hello(prev: ParsedHello) {
    // Use r_println or r_print to 
    //   render the content of ParsedHello
    r_println!("Hello, {}!", *prev)
}
```

> **About r_print**
>
> `r_print!` can only be used inside a `Renderer`. This is because after the `renderer!` macro expands, it injects `r: &mut RenderResult` into the context.
>
> And `r_print!` directly writes content to the value `r`.
> This means: if there is no `&mut RenderResult` named `r` in the context, `r_print!` cannot be used.

## Manual Impl

> ⚠️ WARNING
>
> The following content is not yet fully implemented; currently, only the `renderer!` macro is allowed for implementation.

Similarly, you can also manually implement `Renderer`,

but note that inside the `renderer!` macro, a `register_type!` macro is executed. This macro itself does not expand into any content; it is only used to inform the `gen_program` context that the type exists:

```rust
struct RenderHello;
impl Renderer for RenderHello {
    type Previous = ParsedHello;
    fn render(
        prev: Self::Previous, 
        r: &mut RenderResult
    ) {
        r_println!("Hello, {}!", *prev)
    }
}
 
register_type!(ParsedHello);
```
