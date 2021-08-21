
## 所有权
### 1. 什么是所有权
1. 每个值都有一个变量，这个变量是该值的所有者
2. 每个值同时只能有一个所有者
3. 当所有者超出作用域时，该值将被删除。

当变量走出作用域的时候，rust会自动调用 drop 函数来释放内存。


## 内存



## 分配
1. 变量和数据的交互方式： 移动（Move）
- 基础数据类型的赋值会拷贝两份。
- 引用赋值会导致被引用的变量失效，不能再访问了。

2. 复制
- 如果类型实现了 Copy 这个 trait，那么旧的变量在赋值后仍然可用。
- 如果一个类型或者该类型的一部分实现了 Drop trait，那么 Rust 不允许让它再去实现 Copy trait 了。
- 任何基础的组合类型都可以是 Copy 的， u32/bool/char/f64
- 任何需要分配内存或者某种资源的都不是 Copy 的。
- 如果一个元组里边所有都是可以 Copy 的，那么这个元组是可 Copy 的。


3. 借用
- 其实就是传递变量的引用，由于没有传递变量本身，所以变量没有发生移动，调用的函数并没有获得变量的所有权，所以移动就不会发生。
- 引用后边加上 mut 可以直接改变引用的值。
- 可变引用有一个重要的限制，在特定作用域内，对某一块数据，只能有一个可变的引用。
- 引用的规则（只能满足下列条件之一）：
  - 一个可变的引用
  - 任意数量不可变的引用


### struct

## 7. Package, Crate, Module

### Rust 的代码组织
- 代码组织主要包括：
  - 哪些细节可以暴露，哪些细节是私有的
  - 作用域内哪些名称有效
  - ...
- 模块系统:
  - **Package(包)**： cargo 的特性，让你构建、测试、共享 crate
  - **Crate(单元包)**：一个模块树，它可产生一个Library 或可执行文件
  - **Module(模块)**、use：让你控制代码的组织、作用域、私有路径
  - **Path(路径)**：为 struct、function或module等项命名的方式。


### Packet 和 Crate
- Crate 的类型
  - binary
  - library
- Crate Root
  - 是源代码文件
  - Rust编译器从这里开始，组成你的 Crate 的根 Module
- 一个 Package：
  - 包含 1 个 Cargo.toml，它描述了如何构建这些 Crates
  - 只能包含 0-1 个 Library Crate
  - 可以包含任意数量的 Binary Crate
  - 单至少包含一个 Crate (Library 或 Binary)

### Cargo 的惯例
- src/main.rs:
  - binary crate 的 crate root
  - crate 名与 packge 名相同
- src/lib.rs
  - package 包含一个 library crate
  - library crate 的 crate root
  - crate 名与 package 名相同
- Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary
- 一个 Package 可以同时包含 src/main.rs 和 src/lib.rs
  - 一个 binary crate，一个 library crate
  - 名称与 package 名相同
- 一个 Package 可以有多个 binary crate：
  - 文件放在 src/bin
  - 每个文件是单独的 binary crate

### Crate 的作用
- 将相关功能组合到一个作用域内，便于在项目间进行共享。
  - 防止冲突

### 定义 Module 来控制作用域和私有性
- Module：
  - 在一个 crate 内，将代码进行分组
  - 增加可读性，易于复用
  - 控制项目的私有性。public、private
- 建立 module：
  - mod 关键字
  - 可嵌套
  - 可包含其他项（struct、enum、常量、trait、函数等）的定义
- src/main.rs 和 src/lib.rs 叫做 crate roots:
  - 这两个文件（任意一个）的内容形成了名为 crate 的模块，位于整个模块树的根部。

### 路径（Path）