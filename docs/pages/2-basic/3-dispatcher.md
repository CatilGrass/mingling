<h1 align="center">Dispatcher</h1>
<p align="center">
    Mingling's Basic Components
</p>

---

## Intro

`Dispatcher` is a core concept in **Mingling**, used to dispatch user-input arguments to corresponding types, which are then handled by [Chain](pages/2-basic/4-chain) or [Renderer](pages/2-basic/5-renderer).

To define a `Dispatcher`, it is recommended to use the `dispatcher!` macro provided by `mingling_macros`:

```rust
// User input: your_bin hello
// Will access HelloCommand and
//  dispatch arguments to HelloEntry
dispatcher!("hello",
    HelloCommand => HelloEntry);

// User input: your_bin sub foo
// Will access FooCommand and
//  dispatch arguments to FooEntry
dispatcher!("sub.foo",
    FooCommand => FooEntry);

// Same as above
dispatcher!("sub.bar",
    BarCommand => BarEntry);
```

If you explicitly specify a name in the `gen_program!` macro, for example:

```rust
gen_program!(MyProgram);
```

Then when using the `dispatcher!` macro, you must also explicitly specify the [Program](pages/2-basic/1-program):

```rust
dispatcher!(MyProgram, "hello",
    HelloCommand => HelloEntry);
```

**Tips:** Finally, add the `Dispatcher` you created to the [Program](pages/2-basic/1-program):

```rust
#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(HelloCommand);
    program.with_dispatcher(SubFooCommand);
    program.with_dispatcher(SubBarCommand);
    program.exec().await;
}
```

## Manual Impl

You can also manually implement the basic `Dispatcher` for more fine-grained control. However, compared to the procmacro, it is more cumbersome and cannot intelligently introduce certain traits based on the state of feature flags.

```rust
// Define AddMemberEntry
// Use the `Groupped` derive to 
//  mark AddMemberEntry as a member of ThisProgram
#[derive(Debug, Groupped)]
pub struct AddMemberEntry {
    // Define arguments to store user input
    pub(crate) args: Vec<String>,
}

// Implement the Dispatcher trait
impl Dispatcher<ThisProgram> for AddMemberCommand {
    // Return the node name of this Dispatcher
    fn node(&self) -> Node {
        node!("member.add")
    }
    
    // When executing this Dispatcher, output AddMemberEntry
    fn begin(&self, args: Vec<String>) 
        -> ChainProcess<ThisProgram> 
    {
        AnyOutput::new(AddMemberEntry { args }).route_chain()
    }
    
    // Used to implement the clone trait for this Dispatcher
    fn clone_dispatcher(&self) 
        -> Box<dyn Dispatcher<ThisProgram>>
    {
        Box::new(AddMemberCommand)
    }
}
```

## 💡 Next Page
> **Basic Component** - Chain [Go](./pages/2-basic/4-chain)
