# Changelogs

### Release 0.1.7

#### Fixes:

1. Fixed a build failure on **Windows** caused by `mingling_core/src/program.rs`

#### Features:

1. **\[macros\]** Completed the `clap_parser` feature: **Mingling** now supports parsing input using `clap::Parser`

```rust
#[derive(Groupped, clap::Parser)]
#[dispatcher_clap("your_cmd", YourClapCommand, YourCommandParseError)]
struct YourCommandEntry {
    #[arg(long, short)]
    str_param: String,
    
    #[arg(long, short)]
    path_param: PathBuf,
}
```

2. **\[core\]** Added function `new_with_args` to `Program`
3. **\[core\]** Added function `dispatch_args_dynamic` to `Program`

#### **BREAKING CHANGES**:

1. **\[macros\]** Removed macro `dispatcher_render!` from `mingling_macros`
2. **\[core\]** The `<..., Group>` in `Program<Collect, Group>` no longer requires `std::fmt::Display`
3. **\[core\]** Changed `Program<Collect, Group>` to `Program<Collect>` (merged the Group and Collect types)
4. **\[picker\]** When performing `unpack` or `unpack_directly` on the result of the first `pick` of `Picker`, it no longer returns a tuple

```rust
// Before
#[chain]
fn parse_sth(prev: SomeEntry) -> NextProcess {
    let str: String = Picker::<()>::new(prev.inner)
        .pick_or((), "None")
        .unpack_directly().0;
    let parsed = Something::new(ok);
    parsed
}

// Now
#[chain]
fn parse_sth(prev: SomeEntry) -> NextProcess {
    let str: String = Picker::<()>::new(prev.inner)
        .pick_or((), "None")
        .unpack_directly(); // Directly return the type instead of a tuple
    let parsed = Something::new(ok);
    parsed
}
```

---

### Release 0.1.6 **\[YANKED\]**

`Mingling` 0.1.6 primarily focuses on optimizing the writing experience and code completion.

> [!CAUTION]
>
> This version cannot be built correctly on **Windows**, please do not use this version.

> [!warning]
>
> To align with the `mingling` version, `mingling_core` and `mingling_macros` will skip version `0.1.5` and be released directly as `0.1.6`.

#### Fixes:

1. **\[core\]** Fixed an issue where the `Powershell` completion script could not be used.

#### Features:

1. **\[core\]** Added support for completion descriptions in `Powershell`.
2. **\[core\]** Added more context-based completion functions, such as `filling_argument` and `typing_argument`. For details, see [Docs.rs](https://docs.rs/mingling/0.1.6/mingling/)

#### **BREAKING CHANGES**:

1. **\[macros\]** The `chain!` macro no longer requires explicit type conversion when routing a type to `Chain`.
```rust
// Before
#[chain]
fn proc(_prev: SomeType) -> NextProcess {
    let result = SomeResult::new(());
    result.to_chain()
}

// Now
#[chain]
fn proc(_prev: SomeType) -> NextProcess {
    let result = SomeResult::new(());
    result // No need for `to_chain()`
}
```

2. **\[macros\]** Moved type registration from the `chain!` and `renderer!` macros forward to the `pack!` and `derive Groupped` macros

3. **\[core\]** **\[macros\]** Added an `async` feature, which is disabled by default. `Mingling` no longer forces a dependency on an Async Runtime.

4. **\[picker\]** Changed the signature of `pick_or` from `(..., or: TNext)` to `(..., or: impl Into<TNext>)`
