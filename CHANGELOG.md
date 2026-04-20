# Changelogs

### Release 0.1.6

`Mingling` 0.1.6 primarily focuses on optimizing the writing experience and code completion.

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
