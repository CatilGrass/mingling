该命令运行起来应该是这样：

```bash
~# fruit-mgr <<<<<<<<<< "输入命令"
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager <<<<<<<<<< "输出欢迎词"
~#
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help <<<<<<<<<< "查看 list 命令的帮助"
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>  <<<<<<<<<< "输出帮助"
~#
```
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>
~# fruit-mgr list <<<<<<<<<< "输入命令列出水果"
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>
~# fruit-mgr list
Apple
Banana
Orange  <<<<<<<<<< "输出水果列表"
~#
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>
~# fruit-mgr list
Apple <<<<<<<<<< "当然，我们可以使用 --filter 来过滤出 Apple"
Banana
Orange
~#
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>
~# fruit-mgr list
Apple
Banana
Orange
~# fruit-mgr list --filter A <<<<<<<<<< "过滤出名字含大写 A 的水果"
```
 
---

```bash
~# fruit-mgr
Welcome to use FruitManager
~# fruit-mgr list --help
Description: List all fruits
Usage: fruit-mgr list --filter <NAME>
~# fruit-mgr list
Apple
Banana
Orange
~# fruit-mgr list --filter A
Apple <<<<<<<<<< "只输出 Apple"
~#
```
