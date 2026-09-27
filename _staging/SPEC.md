# rustc 版本笔记批量写作规范（给子任务）

仓库根：`D:\develops\git\github\rust\rust_learn`
任务：为指定 rustc 版本各写一对文件 —— 逐条 changelog 笔记 + 可编译演示代码。

## 数据来源（必读规则）
- 每个版本先 webfetch `https://releases.rs/docs/<版本>/`（text 格式），以页面真实内容为准，**禁止编造特性**。
- 记录页面顶部 "Released on:" 日期；分类照抄页面：Language / Compiler / Libraries / Stabilized APIs / Cargo / Rustdoc / Compatibility Notes（有啥写啥，无则跳过）。
- patch 版本（如 1.45.2、1.97.1）：内容通常很少，照实归类写出全部条目（多为 bugfix/安全修复），md 可短，但必须完整覆盖。
- Stabilized APIs 列表很长时可按条目全部列出（用列表），这是"逐条全写"的要求；Compatibility Notes 也要写。

## 文件 1：笔记 `crates/versions/notes/<版本>.md`（如 `1.92.0.md`）
```markdown
# Rust <版本>（YYYY-MM-DD）

> 来源：[releases.rs/docs/<版本>](https://releases.rs/docs/<版本>/)；演示：`cargo run -p versions --bin rustc_1_x_y`

## Language
- **特性名**：一句中文说明（代码形式 timespec 原样保留）

## Stabilized APIs
- `API::签名` — 中文一句话

## Compatibility Notes
- ...

## 演示对照
| 节 | 主题 |
|---|---|
| 1 | xxx |
（有演示码才列；列节号 = rs 文件里的 println 编号）
```
分类含 Language/Compiler/Libraries/Stabilized APIs/Cargo/Rustdoc/Compatibility Notes 中实际存在者；条目空缺类省略。

## 文件 2：演示 `crates/versions/src/bin/rustc_1_x_y.rs`（版本号点改下划线，如 `rustc_1_92_0.rs`）
```rust
// rustc 1.x.y 演示 —— 特性名概要
fn main() {
    println!("rustc 1.x.y 演示");
    println!("\n1. 特性名");
    // 可运行代码 + println 结果
    println!("\n2. ...");
}
```
硬规则（违反会全仓编译失败）：
1. **必须用当前 rustc 1.98 / edition 2024 可编译且只调用稳定的 API**。写不确定的 API 签名时，宁可少写代码、把机制用 println 讲清楚（注释+输出说明），也不要猜签名。
2. 禁用：已删除的历史 API（`std::old_io`、`Time::precise_time_ns` 等）、`#[feature]`、`unstable`。演示老版本特性时，若 API 已变更/删除，用注释说明历史形态 + 用现行等价物演示。
3. 禁止需要外部 crate 的代码（tokio 仅限确需 async 运行时的版本：1.39.0、1.75.0 异步相关可用 `#[tokio::main]`，其余版本不用）。
4. 每个演示用 `println!("\nN. 中文节名")` 编节；`fn main` 必须；无需 cargo test。
5. 不改任何其他文件；不运行 cargo / git（统一验证）。
6. 注释用中文，讲清"该版本 introduces什么、对比旧行为"。这是教学笔记，代码里允许并鼓励注释。

## 完成后返回（不要写入任何状态文件）
逐行返回：`<版本> | ok | <节名...>`；webfetch 失败的版本单独列出。
