# Rust Edition（版本）新增特性说明

Rust 以 **Edition** 为单位发布语言层面的不兼容变更（约每三年一版）；标准库与 Cargo 的能力则随编译器版本演进、对所有 Edition 通用。
本仓库以 Cargo workspace 分包对应四个 Edition：`crates/e2015`、`crates/e2018`、`crates/e2021`、`crates/e2024` 各持对应 `edition` 值，并提供该版本特性的演示程序。

## Rust 2015（Rust 1.0，2015-05）—— 基线版本

首个稳定版本，后续所有 Edition 的基础：

- **所有权（ownership）与移动（move）语义**：值同一时刻只有一个所有者，赋值/传参会转移所有权
- **借用（borrow）与引用检查**：`&T` 共享借用、`&mut T` 可变借用，编译期保证无数据竞争
- **生命周期（lifetime）**：`<'a>` 标注与借用检查器验证引用有效性
- **trait、泛型、关联类型**：静态分发与组合复用的核心机制
- **代数数据类型与模式匹配**：`enum`、`struct`、`match` 穷尽性检查
- **`macro_rules!` 声明宏**：卫生宏系统
- **2015 风格模块系统**：`use` 以 crate 根为绝对路径；依赖库需 **`extern crate` 声明**（2018 起不再需要）
- 无 `async/await`（异步需回调、线程等手动组合）；无 `dyn Trait`（当时用裸 trait object）

演示：`cargo run -p e2015 --bin edition_2015`

## Rust 2018（Rust 1.31，2018-12）

- **模块系统改进**：路径区分 `crate::` / `self::` / `super::`；`extern crate` 不再必需；嵌套分组导入 `use a::{b, c}`；宏可用 `use` 导入
- **`dyn Trait`**：显式动态分发写法，取代裸 trait object（`Box<Message>` → `Box<dyn Message>`）
- **`async` / `await` 关键字**：异步编程语法糖（稳定于 1.39）
- **匿名生命周期 `'_`**：省略无歧义的生命周期标注（函数参数、`impl` 块等）
- **原始标识符 `r#`**：用关键字作标识符（`r#match`、`r#type`）
- **切片模式**：`[first, second, ..]`、`..=` 范围模式匹配
- **非词法生命周期（NLL）**：借用按控制流计算，变量最后使用后即可修改/释放

演示：`cargo run -p e2018 --bin edition_2018`

## Rust 2021（Rust 1.56，2021-10）

- **闭包精确捕获（disjoint capture）**：闭包只捕获实际用到的字段，部分移动后闭包仍可用（2018 的整体捕获会报 E0382）
- **数组按值 `IntoIterator`**：`arr.into_iter()` 方法调用按值迭代（2018 及以前按切片引用，闭包参数需 `*n`）；`for x in arr` 各版本均按值
- **`panic!` 与 `format!` 行为一致**：单参数必须是字符串字面量，不再把任意表达式当 panic 载荷
- **prelude（预导入）更新**：新增 `TryFrom`、`TryInto`、`FromIterator`，少写导入
- **格式化字符串内联捕获**：`format!("{name}")` / `panic!("{name}")` 直接捕获同名变量
- **保留 `|..|` 语法**：为未来闭包/迭代器扩展预留
- **宏片段预留 `expr_2021`**：为未来 `expr` 片段的行为变更预留说明符

演示：`cargo run -p e2021 --bin edition_2021`

## Rust 2024（Rust 1.85，2025-02）

- **`unsafe` 属性强制化**：`#[unsafe(no_mangle)]`、`#[unsafe(export_name)]`、`#[unsafe(link_section)]`，unsafe 能力必须显式标注
- **`unsafe extern` 块**：外部函数块写为 `unsafe extern "C" { ... }`
- **`unsafe` 函数体内需显式 `unsafe` 块**（`unsafe_op_in_unsafe_fn` 由 allow 变为 error）
- **`static mut` 直接引用变为硬错误**：改用原子类型或 `UnsafeCell`
- **`gen` 成为保留关键字**：为生成器/协程预留（需作标识符时用 `r#gen`）
- **返回位置 `impl Trait` 默认捕获所有生命周期**：配合精确捕获语法 `impl Trait + use<'a>` 显式控制
- **`if let` / 尾表达式临时值作用域收紧**：临时值更早 drop，借用更可预期
- **match 默认绑定模式修正**：`binding @ pattern` 处绑定模式重置，消除歧义
- **never 类型（`!`）回落行为改进**

演示：`cargo run -p e2024 --bin edition_2024`

## 演示文件与包对照

| Edition | 包 | 演示文件 |
|---|---|---|
| 2015 | `e2015` | `crates/e2015/src/bin/edition_2015.rs` |
| 2018 | `e2018` | `crates/e2018/src/bin/edition_2018.rs` |
| 2021 | `e2021` | `crates/e2021/src/bin/edition_2021.rs` |
| 2024 | `e2024` | `crates/e2024/src/bin/edition_2024.rs` |

> 注：本文件位于 `main`；历史 Edition 分支（2015/2018/2021/2024）已归档为 tag `archive/edition-*`。
