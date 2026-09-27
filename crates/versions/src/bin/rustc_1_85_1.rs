// rustc 1.85.1 演示 —— 补丁版：doctest 合并修复、fs::rename on Win10 1607 等
// 无新增稳定语言特性/API，仅以讲解性输出 + 可运行片段说明修复点。
use std::io::Write;

fn main() {
    println!("rustc 1.85.1 演示（patch：修复，无新语言面）");

    println!("\n1. 修复 2024 Edition 的 doctest 合并（讲解性）");
    // doctest-merging：cargo test 会把多个 doctest 合并到一个 crate 里编译以提速；
    // 2024 edition 切换后该特性的属性处理有 bug，1.85.1 修复。
    println!("doctest 合并属 cargo test 内部机制，本演示无法直接观察，修复说明见 md 笔记。");

    println!("\n2. 修复 Windows 10 1607 上 std::fs::rename 的错误");
    // 1.85.0 在该 Windows 版本上 rename 会因权限/句柄问题失败；1.85.1 修复。
    // 实际跑一次 rename 验证路径可用：
    let dir = std::env::temp_dir();
    let from = dir.join("rustlearn_851_rename_from.txt");
    let to = dir.join("rustlearn_851_rename_to.txt");
    std::fs::File::create(&from).unwrap().write_all(b"rename-851").unwrap();
    match std::fs::rename(&from, &to) {
        Ok(()) => println!("rename 成功（Windows 10 1607 上 1.85.0 曾失败）"),
        Err(e) => println!("rename 失败：{e}"),
    }
    let _ = std::fs::remove_file(&to);

    println!("\n3. 其余修复（讲解性）");
    println!("生成文档时放宽部分 target_feature 检查；bootstrap cc 降级支持自定义目标；tarball 构建跳过 submodule 更新。");
}
