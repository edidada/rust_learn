// rustc 1.27.1 演示 —— 安全补丁：rustdoc /tmp 插件漏洞（CVE-2018-1000622）
// 该版本 introduces:
// 修复 rustdoc 运行时会执行 /tmp/rustdoc/plugins 下插件的本地提权风险；
// 另修复 match ergonomics 的两处借用检查 unsoundness（#51415、#49534）。

fn main() {
    println!("rustc 1.27.1 演示");
    println!("\n1. 安全修复（讲解）");
    println!("CVE-2018-1000622：rustdoc 不再执行 /tmp/rustdoc/plugins 目录下的插件");
    println!("\n2. match ergonomics 示意（修复后依旧合法）");
    let maybe: Option<std::vec::Vec<i32>> = Some(vec![1, 2]);
    match &maybe {
        Some(v) => println!("引用上直接匹配 Some(v)，v = {:?}", v),
        None => println!("None"),
    }
}
