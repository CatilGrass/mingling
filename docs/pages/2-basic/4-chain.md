<h1 align="center">Chain</h1>
<p align="center">
    Mingling's Basic Components
</p>

---

## Intro

Like `Dispatcher`, `Chain` is also a core concept in building the entire **Mingling** framework. It is used to receive a dispatch of one type and convert it into another type.

```rust
dispatcher!("hello",
    HelloCommand => HelloEntry);

// Define intermediate type ParsedHello, internally a String
pack!(ParsedHello = String);

// Define chain parse_hello (expands to ParseHello)
// Declare conversion from HelloEntry
#[chain]
async fn parse_hello(prev: HelloEntry) -> NextProcess {
    // Take the inner reference of HelloEntry
    let args = &*prev;

    // Extract the first argument, use default value "World"
    //   if it doesn't exist
    let first = args.first().cloned().unwrap_or_else(|| "World".to_string());

    // Pack the extracted argument into ParsedHello and
    //   dispatch to the next chain
    ParsedHello::new(first).to_chain()
}
```

> **About NextProcess**
>
> `NextProcess` is a marker type in **Mingling**, from `mingling::marker`.
>
> It serves no functional purpose other than to simplify the declaration of chain functions. After the `chain!` macro expands, `NextProcess` will be replaced with `mingling::ChainProcess<ThisProgram>`.

## Manual Impl

> ⚠️ WARNING
>
> The following content is not yet fully implemented; currently, only the `chain!` macro is allowed for implementation.

You can also manually implement the basic `Chain` for finer control.

However, please note that within the `chain!` macro, a `register_type!` macro is executed. This macro does not expand to any content; it only informs the `gen_program` context that this type exists.

```rust
dispatcher!("hello",
    HelloCommand => HelloEntry);
 
pack!(ParsedHello = String);
 
struct ParseHello;
impl Chain<ThisProgram> for ParseHello {
    type Previous = HelloEntry;
    async fn proc(prev: Self::Previous) 
        -> ChainProcess<ThisProgram> 
    {
        let args = &*prev;
        let first = args
            .first()
            .cloned()
            .unwrap_or_else(|| 
                "World".to_string()
            );
        ParsedHello::new(first).to_chain()
    }
}
 
// Register HelloEntry to the context and 
//   assign an ID for it
register_type!(HelloEntry);
```

## 💡 Next Page
> **Basic Component** - Renderer [Go](./pages/2-basic/5-renderer)
