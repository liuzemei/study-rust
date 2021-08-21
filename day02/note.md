
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
- 为了在 Rust 的模块中找到某个条目，需要使用路径。
- 路径的两种形式：
  - 绝对路径： 从 crate root 开始，使用 crate 名 或 字面值 crate
  - 相对路径： 从 当前模块开始，使用 self，super 或当前模块的标识符 
- 路径至少由一个标识符组成，标识符之间使用 `::`。


### 私有边界（privacy boundary）
- 模块不仅可以组织代码，还可以定义私有边界。
- 如果想把 函数 或 struct 设为私有，可以将它放到某个模块中。
- Rust 中所有的条目（函数、方法、struct、enum、模块、常量）都是私有的。
- 父级模块无法访问子模块中的私有条目
- 子模块里可以使用所有祖先模块中的条目

### pub 关键字
- 使用 pub 关键字来讲某些条目标记为公共的
- 同 mod 可以互相调用，不同级别 mod 的话需要添加 pub 关键字来访问

### super 关键字
- **super**： 用来访问父级模块路径中的内容，类似于文件系统中的 **..**

### pub struct
- pub 放在 struct 前：
  - struct 是公共的
  - struct 的字段默认是私有的
- struct 的字段需要单独设置 pub 来变成公有的。

### pub enum
- 枚举里的所有值都会被标记为 pub


### use 关键字
- 可以使用 use 关键字将路径导入到作用域内
  - 仍遵循私有性规则


### use 的习惯用法
- 函数：将函数的父级模块引入作用域，然后通过 父级模块 :: 来访问函数。
- struct，enum，其他：指定完整路径，不通过父级搞了。
- 同名条目：指定到父级

### as 关键字
- 起别名

### 使用 pub use 重新导出名称
- 使用 use 将路径（名称）导入到作用域内后，该名称在此作用域内是私有的。
- pub use: 重导出。
  - 将条目引入作用域
  - 该条目可以被外部代码引入到他们的作用域。


### 使用外部包 （package）
1. Cargo.toml 添加依赖的包（package）
  - https://crates.io
2. use 将特定条目引入作用域

- 标准库（std）也被当作外部包了
  - 不需要修改 Cargo.toml 来包含 std
  - 需要使用 use 将 std 中的特定条目引入当前作用域


### 使用 嵌套路径 清理大量的 use 语句
- 如果使用同一个包或模块下的多个条目
- 可以使用嵌套路径在同一行内将上述条目进行引入
  - 路径相同的部分::{路径差异的部分}
- 可以使用 {self} 引入自己
- 可以使用 * 来引入所有的公共条目
  - 场景： 
  - 1. 测试。将所有被测试代码引入到
  - 2. 有时被用于预导入（prelude）模块

### 将模块内容放到其他文件中
- 模块定义时，如果模块名后边是 `;`，而不是代码块：
  - Rust 会从与模块同名的文件中加载内容
  - 模块树的结构不会变化
- 随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中。


## 8. 常见的集合
本章内容：
- Vector
- String
- HashMap


### Vector
- Vec<T> 叫做 vector
  - 由标准库提供
  - 可存储多个值
  - 只能存储相同类型的数据
  - 值在内存中连续存放

### 所有权和借用规则
- 不能在同一作用域内同时拥有可变和不可变引用


### HashMap
- 对于实现了 Copy trait 的类型，值会被复制到 HashMap 中
- 对于拥有所有权的值，值会被移动，所有权会转移给 HashMap
- 如果将值的引用传递，值就还能够访问。

- insert(k,v) 
- entry(k).or_insert(v)
- 