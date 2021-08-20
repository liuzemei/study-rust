### 1. 编译/cargo简介
```shell
# 编译文件
rustc xxxx.rs

# 生成 cargo 项目
cargo new hello_cargo
cd hello_cargo
cargo run # build -> exec
cargo build
cargo check # 检查代码是否能够通过编译

cargo bulid --release # 代码会运行的更快，但是编译时间更长
```

### 2. 搞个猜数字游戏
1. 默认声明的变量都是 不可变的
2. 如果要声明一个可变的变量 需要用 `mut` 来修饰
3. `::` 可以访问静态函数（实例上的函数）
4. 可以重复对变量进行声明。

### 3. 数据类型
1. 基础类型： 整数/浮点数/布尔类型/字符类型
2. 复合类型： Tuple(元组)


### 4. 函数
1. 声明函数使用 fn 关键字
2. 函数的参数需要指定类型
3. -> 符号后边声明函数返回值的类型， 返回值就是函数题最后一个表达式的值。
4. 若想提前返回，需要使用 `return` 关键字，并指定一个值。