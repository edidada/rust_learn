// rustc 1.26.2 演示 —— patch 版：match ergonomics soundness 修复
// 该版本 introduces:
// 仅一个修复：借用检查器修补 "模式自动解引用（match ergonomics）"
// 引入的潜在内存安全漏洞（unsoundness）。无 API/语言新增。
// 1.26.0 -> 1.26.1 -> 1.26.2 三连补丁都围绕误稳定与 soundness 收口。

fn main() {
    println!("rustc 1.26.2 演示");
    println!("\n1. patch 版定位");
    println!("修复：match ergonomics 下借用检查的 unsoundness（对照 &Option 等引用型数据的模式匹配）");
    println!("\n2. match ergonomics 正常用例（修复后依旧合法）");
    let maybe: Option<(i32, &str)> = Some((3, "三"));
    match &maybe {
        Some((n, label)) => println!("自动解引用：{} = {}", n, label),
        None => println!("None"),
    }
    let m = std::collections::HashMap::from([("a", 1i32)]);
    for (k, v) in &m {
        println!("遍历 &HashMap 自动解引用：{} -> {}", k, v);
    }
}
