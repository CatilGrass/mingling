<h1 align="center">初次上手！</h1>
<p align="center">
    使用 <b>Mingling</b> 开发简易水果名搜索命令行程序
</p>

## 前言

在本篇示例中，我将介绍如何使用 **Mingling** 开发基本的命令行程序，它包括：

1. `fruit-mgr list` 命令
2. `fruit-mgr list --help` 帮助页面

<iframe
    src="../play/play.html?tur=zh_CN/2-fruit-mgr-expect.md&amp;title=水果管理器预期效果"
    height="580px">
</iframe>

## 一、导入依赖

首先，请确保您的项目依赖 **Mingling**

若未导入依赖，您可以在 `Cargo.toml` 添加如下内容：

```toml
[dependencies.mingling]
version = "0.1.9"
features = [""]
```
 
## 二、编写 `main.rs`，创建基本的项目

在 `src/main.rs` 中，我们将编写程序的入口函数：

<iframe
    src="../play/play.html?tur=zh_CN/2-writing.md&amp;title=编写入口"
    height="500px">
</iframe>

## 三、注册 `list` 子命令

<iframe
    src="../play/play.html?tur=zh_CN/2-writing1.md&amp;title=注册 List 子命令"
    height="550px">
</iframe>

## 四、实现 `EntryList` 行为

<iframe
    src="../play/play.html?tur=zh_CN/2-writing2.md&amp;title=实现 EntryList 行为"
    height="860px">
</iframe>

<p align="center" style="font-size: 0.85em; color: gray;">
    Written by @Weicao-CatilGrass
</p>

## 五、渲染 `ResultFruits`

在上一步中，`handle_entry_list` 链接收 `EntryList`，并输出了 `ResultFruits`。

接下来我们就要为 `ResultFruits` 渲染出最终的结果，使整个程序像下图这样排列：

<center>
    <img src="../res/graph/2-1.drawio.svg"/>
</center>

让我们开始编写代码吧：

<iframe
    src="../play/play.html?tur=zh_CN/2-writing3.md&amp;title=渲染 ResultFruits"
    height="860px">
</iframe>

### 💡 关于 `r_println!` 宏

`r_println!` 宏并不是对 `println!` 的包装

它是为 `#[renderer]` 宏中 **隐式注入** 的 `&mut RenderResult` 值提供的便捷访问方案。

在 `r_println!` 调用时，不会立刻向 stdout 输出信息，而是会被累计到 `&mut RenderResult` 中，在程序结束时整体输出。

## 六、编译并运行

现在您已经完成了 `list` 命令，现在可以使用 `cargo run` 运行您的程序：

```bash
cargo run -- list
```
 
您将会看到如下输出：

```
Apple
Banana
Orange
```
 
<details>
<summary>完整代码</summary>

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
 
#[renderer]
fn render_fruits(result: ResultFruits) {
    let vec: &Vec<String> = &*result;
    for fruit in vec {
        r_println!("{}", fruit)
    }
}
 
gen_program!();
 
```
 
</details>

<p align="center" style="font-size: 0.85em; color: gray;">
    Written by @Weicao-CatilGrass
</p>
