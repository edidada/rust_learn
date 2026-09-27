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


```
Sequences: Vec, VecDeque, LinkedList
Maps: HashMap, BTreeMap
Sets: HashSet, BTreeSet
Misc: BinaryHeap
```

---

## 项目构建信息

### 系统环境
- **操作系统**: macOS Intel 12.7
- **Rust版本**: 最新稳定版
- **项目版本**: 0.1.0
- **Edition**: 2024

### 可执行文件列表 (共150个)

#### Collections模块 (8个)
- `vec` - Vec集合演示
- `vecdeque` - VecDeque双端队列演示
- `linkedlist` - 链表演示
- `box_array` - Box数组演示
- `btreemap` - BTreeMap有序映射
- `btreeset` - BTreeSet有序集合
- `hashmap` - HashMap哈希映射
- `hashset` - HashSet哈希集合

#### Keywords模块 (37个)
Rust关键字示例程序：
- `keywords_as`, `keywords_async`, `keywords_await`, `keywords_become`
- `keywords_break`, `keywords_const`, `keywords_continue`, `keywords_crate`
- `keywords_dyn`, `keywords_else`, `keywords_enum`, `keywords_extern`
- `keywords_false`, `keywords_fn`, `keywords_for`, `keywords_if`
- `keywords_impl`, `keywords_in`, `keywords_let`, `keywords_loop`
- `keywords_match`, `keywords_mod`, `keywords_move`, `keywords_mut`
- `keywords_pub`, `keywords_ref`, `keywords_return`, `keywords_self`
- `keywords_selfty`, `keywords_static`, `keywords_struct`, `keywords_super`
- `keywords_trait`, `keywords_true`, `keywords_type`, `keywords_union`
- `keywords_unsafe`, `keywords_use`, `keywords_where`, `keywords_while`

#### Threads模块 (6个)
- `basic_thread` - 基础线程操作
- `thread_local` - 线程本地存储
- `thread_sync` - 线程同步
- `thread_attributes` - 线程属性
- `thread_advanced` - 高级线程
- `thread_complete` - 完整线程示例

#### Exercises练习模块 (95个)
按主题分类的练习程序：
- **01_variables**: variables1-6 (变量基础)
- **02_functions**: functions1-5 (函数)
- **03_if**: if1-3 (条件语句)
- **04_primitive_types**: primitive_types1-6 (基本类型)
- **05_vecs**: vecs1-2 (向量)
- **06_move_semantics**: move_semantics1-5 (移动语义)
- **07_structs**: structs1-3 (结构体)
- **08_enums**: enums1-3 (枚举)
- **09_strings**: strings1-4 (字符串)
- **10_modules**: modules1-3 (模块)
- **11_hashmaps**: hashmaps1-3 (哈希映射)
- **12_options**: options1-3 (Option类型)
- **13_error_handling**: errors1-6 (错误处理)
- **14_generics**: generics1-2 (泛型)
- **15_traits**: traits1-5 (特质)
- **16_lifetimes**: lifetimes1-3 (生命周期)
- **17_tests**: tests1-3 (测试)
- **18_iterators**: iterators1-5 (迭代器)
- **19_smart_pointers**: arc1, box1, cow1, rc1 (智能指针)
- **20_threads**: threads1-3 (线程练习)
- **21_macros**: macros1-4 (宏)
- **22_clippy**: clippy1-3 (Clippy工具)
- **23_conversions**: as_ref_mut, from_into, from_str, try_from_into, using_as (类型转换)
- **quizzes**: quiz1-3 (测验)

### 运行示例

```bash
# 运行Vec集合示例
./target/debug/vec

# 运行异步关键字示例
./target/debug/keywords_async

# 运行基础线程示例
./target/debug/basic_thread
```

### Git分支信息

项目包含以下分支，用于展示不同Rust Edition的特性：
- `main` - 主分支
- `master` - 备用主分支
- `2024` - Rust 2024 Edition 特性
- `2021` - Rust 2021 Edition 特性
- `2018` - Rust 2018 Edition 特性
- `2015` - Rust 2015 Edition 特性

### 编译命令

```bash
# 编译所有可执行文件
cargo build --bins

# 编译并运行特定程序
cargo run --bin vec
cargo run --bin keywords_async
cargo run --bin basic_thread
```

### 依赖项
- `thread_local` = "1.1"
- `tokio` = { version = "1.0", features = ["full"] }

---

## Rust Edition 特性演示

项目包含4个版本特性演示程序，展示不同Rust Edition的主要特性：

### 运行版本特性演示

```bash
# Rust 2015 Edition - 基础特性
cargo run --bin edition_2015

# Rust 2018 Edition - 模块系统、async/await、dyn Trait
cargo run --bin edition_2018

# Rust 2021 Edition - 闭包捕获、panic宏、数组IntoIterator
cargo run --bin edition_2021

# Rust 2024 Edition - 临时生命周期、宏改进、类型推断
cargo run --bin edition_2024
```

### 各版本主要特性

#### Rust 2015 (Rust 1.0 - 2015年5月)
- **所有权系统** - Rust的核心内存管理模型
- **借用和引用** - 安全的引用机制
- **生命周期** - 编译时引用有效性检查
- **模式匹配** - 强大的match表达式
- **特质系统** - 接口抽象机制
- **错误处理** - Result和Option类型
- **宏系统** - 声明宏支持

#### Rust 2018 (2018年12月)
- **模块系统改进** - 简化模块导入，统一路径语法
- **async/await** - 异步编程语法糖
- **dyn Trait** - 显式动态分发
- **匿名生命周期** - `'_`简化生命周期标注
- **原始标识符** - `r#`前缀使用关键字作为标识符
- **切片模式匹配** - 数组/向量模式匹配改进
- **非词法生命周期** - 改进的借用检查器

#### Rust 2021 (2021年10月)
- **闭包捕获改进** - 只捕获实际使用的字段
- **panic!宏一致性** - 统一的panic宏行为
- **数组IntoIterator** - 数组直接实现IntoIterator trait
- **保留语法** - 为未来扩展预留语法
- **格式化字符串** - 直接捕获变量 `{variable}`
- **预导入模块更新** - 新增常用trait和类型

#### Rust 2024 (2024年)
- **临时生命周期延长** - 更灵活的临时值生命周期
- **宏系统改进** - 更好的错误诊断和调试
- **模式匹配改进** - 更灵活的匹配守卫
- **类型推断改进** - 更智能的泛型推断
- **标准库新特性** - 新增Iterator方法和API
- **Cargo改进** - 更快的编译速度和更好的诊断
- **异步编程改进** - 异步trait和异步迭代器
