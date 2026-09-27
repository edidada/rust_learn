# README
Rust 与 C++、Java 的版本演进机制完全不同。一句话概括：**Rust 是“编译器版本（Cargo/rustc）+ 版次（Edition）”双轨制；C++ 是单一标准版本；Java 是单一语言版本。**

### Rust：双轨制（编译器版本 + Edition）

Rust 的新特性通过 **6 周一次的 rustc/Cargo 版本更新**持续引入，**大多数特性对所有 Edition 都可用**。Edition（2015/2018/2021/2024）则是一个**可选的、约每 3 年一次的“打包”机制**，只包含**会破坏向后兼容的少数变更**（如新增关键字、改变解析规则）。

**这意味着：你用 Rust 1.90 编译器，可以选择 Edition 2015、2018、2021 或 2024。Edition 只影响编译器“如何解析你的代码”，不阻止你使用最新的语言特性。**

*   **Rust 2015**：初始版本。宏系统较为基础。
*   **Rust 2018**：简化了模块路径系统；引入 `impl Trait` 语法；支持原始标识符（`r#`）。
*   **Rust 2021**：闭包捕获优化（只捕获用到的字段，不再捕获整个结构体）；将 `TryInto`、`TryFrom`、`FromIterator` 加入 Prelude（预导入）。
*   **Rust 2024**：`let chains`（链式 `let`，如 `if let ... && let ...`）；`unsafe` 属性的强制要求；临时值作用域规则的调整。

### C++：单一标准版本

C++ 通过 **ISO 标准**的更新来引入特性，**没有“编译器版本”与“语言版本”的分离**。C++11、17、20、23、26 就是语言本身的新版本，新特性与标准强绑定。

*   **C++11**：Lambda 表达式、`auto`、智能指针、范围 for 循环、可变参数模板等。
*   **C++17**：结构化绑定、`if constexpr`、折叠表达式、文件系统库等。
*   **C++20**：概念（Concepts）、协程（Coroutines）、模块（Modules）、范围（Ranges）、三路比较 `<=>`。
*   **C++23**：`std::print`、`std::expected`、多维下标、范围视图增强。
*   **C++26**：编译期反射、合约编程、`std::execution`、内存安全强化。

### Java：单一语言版本（带部分共存）

Java 也通过**单一语言版本**引入特性，但机制介于两者之间：新版本会加入新特性，同时通过**预览特性（Preview）**和**孵化器（Incubator）**机制，让部分特性可以提前试用，最终在后续版本中固化。

*   **Java 8**：Lambda 表达式、Stream API、新日期时间 API。
*   **Java 11**：`var` 局部变量类型推断、新的 HTTP Client。
*   **Java 17**：密封类（Sealed Classes）、模式匹配 for `switch`（预览）。
*   **Java 21**：虚拟线程（Virtual Threads）、记录模式（Record Patterns）、分代 ZGC。

### 总结

| 语言 | 版本机制 | 核心特点 |
| :--- | :--- | :--- |
| **Rust** | **双轨制** | **编译器持续更新 + 可选 Edition 打包不兼容变更**。大多数新特性与 Edition 无关。 |
| **C++** | **单一标准** | 标准版本即语言版本，新特性完全由 ISO 标准定义。 |
| **Java** | **单一版本 + 预览** | 新特性随版本发布，但可通过预览机制提前试用。 |

所以，你之前遇到的 `edition = "2015"` 与 `autobins` 警告，就是 Rust 双轨制的体现：`autobins` 的行为变化是 **Cargo（编译器工具链）层面**的演进，而 `edition` 决定了你的项目**是否主动选择**接受这种新的、可能不兼容的解析行为。

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
