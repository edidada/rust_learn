// rustc 1.71.1 演示 —— patch 版本：CVE-2023-38497（Cargo umask）等（文字说明）
fn main() {
    println!("rustc 1.71.1 演示");
    println!("\n1. CVE-2023-38497：Cargo 解压依赖未遵守 umask");
    println!("  - 本地用户可篡改 Cargo 解压出的文件，进而影响同机其他用户的构建产物;");
    println!("  - 1.71.1 起解压遵循 umask 设置。属于构建工具行为修复。");
    println!("\n2. 其他修复");
    println!("  - bash 补全（rustup 用户）；调用 borrow() 不再误报 suspicious_double_ref_op;");
    println!("  - 修复两个编译器 ICE；修复源码 tarball 构建。");
}
