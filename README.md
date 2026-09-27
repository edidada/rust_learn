# README

```shell
3 |     let x: i32;
|         - binding declared here but left uninitialized
4 |
5 |     println!("Number {x}");
|                       ^ `x` used here but it isn't initialized
```


```
error[E0384]: cannot assign twice to immutable variable `x`
--> src\exercises\01_variables\variables4.rs:6:5
|
3 |     let  x = 3;
|          - first assignment to `x`
...
6 |     x = 5; // Don't change this line
|     ^^^^^ cannot assign twice to immutable variable
|
```

## 2015

Rust 首个稳定 Edition：所有权、借用、生命周期、`mod` 模块系统等核心语言特性的定型版本。
演示：`cargo run --bin edition_2015`。

## 2018

模块路径改进（`crate::` 路径、不再需要 `extern crate`）、`dyn Trait`、`async/await` 关键字、匿名生命周期 `'_`、原始标识符 `r#`、切片模式匹配。
演示：`cargo run --bin edition_2018`。

## 2021

**问题：** 同一行代码，2018 下要写 `*n`，2021 下不用：

```rust
// 2018
let division_results = numbers.into_iter().map(|n| divide(*n, 27));
// 2021
let division_results = numbers.into_iter().map(|n| divide(n, 27));
```

**真正原因（不是"闭包自动解引用"）：** 数组方法调用 `.into_iter()` 的按值/按引用语义由 Edition 门控。
Rust 1.53 起数组实现了按值的 `IntoIterator for [T; N]`，但为兼容旧代码，2021 之前 `.into_iter()` 这种**方法调用**仍按切片迭代（`n: &i32`，故需 `*n`）；从 2021 起方法调用按值迭代（`n: i32`）。
注意 `for x in arr` 在所有 Edition 下都按值迭代——这属于迭代器行为变更，与闭包捕获、`Deref` 自动解引用均无关，不存在"编译器自动从 `&i32` 解引用为 `i32`"这回事。

跨 Edition 通用的写法：

```rust
let division_results = numbers.iter().copied().map(|n| divide(n, 27));
```

**2021 闭包的真正改进 = 精确捕获（precise capture）：** 闭包只捕获实际用到的字段，部分移动后闭包仍可用（2018 的整体捕获会触发 E0382）。完整演示见 `cargo run --bin edition_2021` 第 1、7 节。

迁移：`cargo fix --edition` 可自动完成大部分迁移。

## 2024

最新 Edition：`unsafe` 属性强制化（如 `#[unsafe(no_mangle)]`）、`gen` 保留字、闭包捕获规则进一步简化、部分生命周期语法收紧。
演示见 `src/bin/edition_2024.rs`（仅 2024 分支提供）。
