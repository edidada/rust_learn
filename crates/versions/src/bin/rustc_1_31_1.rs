// rustc 1.31.1 演示 —— patch 版：netbsd 构建修复 + RLS 修复
// 该版本 introduces:
// 1. 修复 powerpc-unknown-netbsd 目标的构建失败。
// 2. 修复 RLS：go-to-definition 失效、hover 无限循环。
// 无语言/库变化。

fn main() {
    println!("rustc 1.31.1 演示");
    println!("\n1. patch 版定位");
    println!("修复 powerpc-unknown-netbsd 构建；修复 RLS 跳转定义与 hover 无限循环");
    println!("\n2. Edition 2018 路径规则复检（仍正常）");
    mod inner {
        use crate::ping;
        pub fn call() {
            ping();
        }
    }
    inner::call();
}

fn ping() {
    println!("crate::ping -> ok");
}
