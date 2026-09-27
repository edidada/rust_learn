// rustc 1.27.2 演示 —— patch 版：match ergonomics soundness 修复 #52213
// 该版本 introduces:
// 仅修复借用检查器在 match ergonomics 下的又一处 unsoundness（#52213）。
// 1.26.1 / 1.27.1 / 1.27.2 均为该主题的连续收口。

fn main() {
    println!("rustc 1.27.2 演示");
    println!("\n1. patch 版定位");
    println!("修复：match ergonomics unsoundness（issue #52213），无新增 API");
    println!("\n2. match ergonomics 示意");
    let maybe: Option<String> = Some("ok".to_string());
    match &maybe {
        Some(s) => println!("Some(s) 在 &Option 上直接解构，s = {}", s),
        None => println!("None"),
    }
}
