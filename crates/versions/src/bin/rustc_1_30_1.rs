// rustc 1.30.1 演示 —— patch 版：rustdoc 溢出 ICE 修复
// 该版本 introduces:
// 1. 修复 rustdoc 中触发溢出型 ICE（内部编译错误）的问题。
// 2. MSYS 终端里 Cargo 进度条限宽 60。
// 无语言/库变化。

fn main() {
    println!("rustc 1.30.1 演示");
    println!("\n1. patch 版定位");
    println!("修复 rustdoc 溢出 ICE；MSYS 下 Cargo 进度条限宽 60 列");
    println!("r#for 等 1.30 特性照常可用：");
    let r#fn = 1;
    println!("let r#fn = {}", r#fn);
}
