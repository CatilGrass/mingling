好的，现在开始编写 `ResultFruits` 的渲染器

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
<<<<<<<<<< "我们将在此处编写渲染器代码"
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] <<<<<<<<<< "和 #[chain] 类似，渲染器函数叫 #[renderer]"
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits( <<<<<<<<<< "编写函数体"
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits <<<<<<<<<< "声明接收 ResultFruits 类型"
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {} <<<<<<<<<< "一般情况下，渲染器什么都不用返回"
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    <<<<<<<<<< "接下来在此处编写渲染逻辑"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = <<<<<<<<<< "首先我们先解包 ResultFruits"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = <<<<<<<<<< "pack! 宏会自动为 ResultFruits 实现 AsRef trait"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = result.as_ref(); <<<<<<<<<< "所以此处可以使用 as_ref()"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; <<<<<<<<<< "当然，deref 也可以解包 ResultFruits"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; 
    for fruit in vec { <<<<<<<<<< "接下来，循环遍历 ResultFruits 中的水果字符串"
} 
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; 
    for fruit in vec {
        <<<<<<<<<< "在循环中打印内容"
    }
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; 
    for fruit in vec {
        r_println!("{}", fruit) <<<<<<<<<< "使用 r_println 在循环中打印内容"
    }
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*; <<<<<<<<<< "r_println! 由 prelude 提供"
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer] 
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; 
    for fruit in vec {
        r_println!("{}", fruit)
    }
}
 
gen_program!();  
```
 
---

至此 `ResultFruits` 的渲染逻辑便已完成

```rust
use mingling::prelude::*;
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    // ...
}
 
#[renderer]
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result; 
    for fruit in vec {
        r_println!("{}", fruit)
    }
}
 
gen_program!();  
```
