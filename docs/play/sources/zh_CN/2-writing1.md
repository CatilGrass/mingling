我们刚刚创建了基本的入口，现在编写子命令: `list`

```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new();
 
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
fn main() {
    let mut program = ThisProgram::new();
    <<<<<<<<<< "所有的资源将在此处注册"
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
<<<<<<<<<< "让我们在此处注册一个分发器，将 list 命令分发到指定的入口"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!( <<<<<<<<<< "编写"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", <<<<<<<<<< "编写命令名称"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList <<<<<<<<<< "编写分发器名称"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList <<<<<<<<<< "编写入口名称"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*;
 
dispatcher!("list", CMDList => EntryList); <<<<<<<<<< "这样，list 命令的分发器 CMDList 就被注册了"
 
fn main() {
    let mut program = ThisProgram::new();
    
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

```rust
use mingling::prelude::*; <<<<<<<<<< "dispatcher! 宏由 prelude 提供"
 
dispatcher!("list", CMDList => EntryList); 
 
fn main() {
    let mut program = ThisProgram::new();
    
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
    <<<<<<<<<< "Ok, 现在在此处为程序添加分发器"
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
    program.with_dispatcher( <<<<<<<<<< "使用 with_dispatcher 为程序添加分发器"
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
    program.with_dispatcher(CMDList); <<<<<<<<<< "添加刚创建的分发器 CMDList"
    program.exec_and_exit();
}
 
gen_program!(); 
```
 
---

至此，`list` 子命令便已被注册到程序

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
