// rustc 1.43.1 演示 —— patch 版 bugfix：openssl CVE / AVX-512 稳定化修复 / cargo package --list
// 说明：本版无新增语言/库特性，以下节对应笔记中的 Patch Fixes，用现行稳定写法描述修复内容。

fn main() {
    println!("rustc 1.43.1 演示（patch：仅修复，无新增稳定 API）");

    println!("\n1. openssl-src 升级到 1.1.1g（CVE-2020-1967）");
    // 背景：Rust 使用的 OpenSSL 静态源升级，修复 CVE-2020-1967（SSL_get_verify_result
    // 等场景的段错误等漏洞）。对一般 std-only 程序无影响；使用出网/加密依赖的环境需更新工具链。
    println!("机制：工具链内嵌的 openssl-src 从 1.1.1f 升级到 1.1.1g。");
    println!("含 openssl 依赖的项目通过 cargo 更新 rustup 工具链自动获得修复，同版本不必改代码。");

    println!("\n2. AVX-512 特性（target feature）稳定化修复");
    // 背景：1.43.0 稳定了一部分 target_feature 属性，但 AVX-512 相关的特性稳定化不完整，
    // 本版修复相关定义；这些是编译目标特性，运行期没有可观察的新 API。
    println!("机制：AVX-512 系列是 x86 平台的 SIMD 扩展；1.43.1 修复其稳定化声明，");
    println!("可在代码里检测支持（示例为演示，运行在具体 CPU 上结果各异）：");
    let has_avx = std::arch::is_x86_feature_detected!("avx2"); // 稳定的运行时检测
    println!("当前主机支持 avx2 = {}（AVX-512 的检测与启用属编译期 target feature）", has_avx);

    println!("\n3. cargo package --list 对未发布依赖可用");
    // 背景：`cargo package --list` 之前在依赖未发布（如 path/git 依赖）时报错，本版修复。
    println!("机制：`cargo package --list` 列出将要打包进 .crate 的文件；");
    println!("修复后在拥有 path/git 等 unpublished 依赖的 workspace 中也能正常列出文件。");
    println!("命令行示例（不在演示中执行）：cargo package --list");
}
