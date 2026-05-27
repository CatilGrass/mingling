现在让我们为其创建行为注册的 `list` 子命令添加入口行为

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
<<<<<<<<<< "在此处创建链，用于接收 CMDList 传入的 EntryList"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain] <<<<<<<<<< "编写"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list( <<<<<<<<<< "创建函数体"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list(prev: EntryList <<<<<<<<<< "声明接收 EntryList 类型"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {} <<<<<<<<<< "返回 Next，这代表下一个类型"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {} <<<<<<<<<< "Next 是 ChainProcess<ThisProgram> 的别名"
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  <<<<<<<<<< "Next 由 gen_program!() 生成"
```
 
---

我们需要一个包装类型用来包装 `handle_entry_list` 的结果

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
<<<<<<<<<< "我们在此处定义一个包装类型"
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits <<<<<<<<<< "使用 pack! 宏快速创建一个包装类型"
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  
```
 
---


```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>); <<<<<<<<<< "将 ResultFruits 包装为 Vec<String>"
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*; <<<<<<<<<< "pack! 由 prelude 提供"
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
 
}
 
gen_program!();  
```
 
---

现在，继续编辑 `handle_entry_list` 的行为

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    <<<<<<<<<< "在此处硬编码水果列表"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    let fruits = <<<<<<<<<< "编写"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    let fruits = vec![ <<<<<<<<<< "编写"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; <<<<<<<<<< "编写"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; 
    <<<<<<<<<< "接下来，在此处将 Vec<String> 包装成 ResultFruits 并返回"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(prev: EntryList) -> Next {
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; 
    ResultFruits::new(fruits) <<<<<<<<<< "编写"
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]             vvvv <<<<<<<<<< "因为没使用 prev 传入的参数，所以将其忽略"
fn handle_entry_list(prev: EntryList) -> Next { 
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; 
    ResultFruits::new(fruits)
}
 
gen_program!();  
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next { 
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; 
    ResultFruits::new(fruits)
}
 
gen_program!();  
```
 
---

好的，现在 `EntryList` 的行为便编写完成

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDList);
    program.exec_and_exit();
}
 
pack!(ResultFruits = Vec<String>);
 
#[chain]
fn handle_entry_list(_prev: EntryList) -> Next {
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
    ]; 
    ResultFruits::new(fruits)
}
 
gen_program!();  
```
