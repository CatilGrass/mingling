这是 `main.rs` 的默认内容，让我们先删除默认的 `Hello, World!`

```rust
 
 
fn main() {
    println!("Hello, World!");
}
```
 
---

```rust
 
 
fn main() {
    println!("Hello, World!"); <<<<<<<<<< "删除 Hello, World!"
}
```
 
---

```rust
 
 
fn main() {
    <<<<<<<<<< "删除 Hello, World!"
}
```
 
---

接下来，让我们构建基本的 **Mingling** 程序入口

```rust
 
 
fn main() {
    
}
```
 
---

```rust
<<<<<<<<<< "在此处引入 Mingling 的预导入模块"
 
fn main() {
    
}
```
 
---
```rust
use mingling::prelude::*; <<<<<<<<<< "编写"
 
fn main() {
    
}
```
 
---

```rust
use mingling::prelude::*;
 
fn main() {
    
}
 
<<<<<<<<<< "在此处引入 gen_program!() 用于生成 ThisProgram"
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    
}
 
gen_program!(); <<<<<<<<<< "编写"
```
 
---


```rust
use mingling::prelude::*; <<<<<<<<<< "gen_program!() 由 prelude 提供"
 
fn main() {
    
}
 
gen_program!();
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    
}
 
gen_program!(); <<<<<<<<<< "gen_program!() 必须放在根模块的结尾，这是宏展开的限制"
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    <<<<<<<<<< "在此处使用 ThisProgram 创建程序"
}
 
gen_program!(); 
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new(); <<<<<<<<<< "编写"
}
 
gen_program!(); 
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new(); 
}
 
gen_program!(); <<<<<<<<<< "ThisProgram 由 gen_program!() 生成"
```
 
---

```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new();
 
    <<<<<<<<<< "在此处执行程序并退出"
}
 
gen_program!(); 
```
 
---


```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new();
 
    program.exec_and_exit(); <<<<<<<<<< "编写"
}
 
gen_program!(); 
```
 
---

至此， **Mingling**的基本入口就搭建完成

```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new();
 
    program.exec_and_exit();
}
 
gen_program!(); 
```
