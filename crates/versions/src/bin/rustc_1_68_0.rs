// rustc 1.68.0 演示 —— pin!、Vec->VecDeque O(1)、PathBuf DerefMut、sparse registry 说明
fn main() {
    println!("rustc 1.68.0 演示");

    // ============================================================
    println!("\n1. std::pin::pin! 宏");
    // 1.68 稳定 pin!：把局部值固定在栈上，得到 Pin<&mut T>，无需 Box::pin
    let mut value = String::from("hello");
    let mut pinned = std::pin::pin!(value);
    // pinned 是 Pin<&mut String>；String 本身不需要 Pin，这里演示类型可用
    let r: &mut String = pinned.as_mut().get_mut();
    r.push_str(" pin!");
    println!("  pin! 固定后可安全取回可变引用并修改: {}", r);

    // ============================================================
    println!("\n2. Vec -> VecDeque 转换 O(1) 保证");
    // 1.68 起 From<Vec<T>> for VecDeque<T> 承诺 O(1)
    use std::collections::VecDeque;
    let v: Vec<i32> = (0..5).collect();
    let dq: VecDeque<i32> = v.into();
    println!("  VecDeque::from(Vec) = {:?}", dq); // [0, 1, 2, 3, 4]
    println!("  front = {:?}, back = {:?}", dq.front(), dq.back());

    // ============================================================
    println!("\n3. PathBuf 的 DerefMut 与 MAIN_SEPARATOR_STR");
    use std::path::{PathBuf, MAIN_SEPARATOR_STR};
    let mut pb = PathBuf::from("a");
    // 1.68 起 PathBuf 实现 DerefMut，可直接调用 Path 的可变/消耗方法
    pb.push("b"); // 原本 push 本来就有；这里演示 DerefMut 通道
    println!("  PathBuf = {:?}", pb);
    println!("  MAIN_SEPARATOR_STR = {:?}（Windows 下是 \\）", MAIN_SEPARATOR_STR);
    println!("  分隔符: {}", std::path::MAIN_SEPARATOR);

    // ============================================================
    println!("\n4. impl From<bool> for f32/f64");
    let f: f64 = true.into();
    let g: f32 = false.into();
    println!("  true -> f64 = {}, false -> f32 = {}", f, g);

    // ============================================================
    println!("\n5. Weak 的 Debug 放宽");
    use std::rc::Rc;
    use std::sync::Weak;
    let w: Weak<String> = Weak::new(); // 空 Weak，无 T: Debug 约束也能打印
    println!("  空 Weak<String> 的 Debug = {:?}", w);
    let strong = Rc::new(String::from("x"));
    let w2 = Rc::downgrade(&strong);
    println!("  有值 Weak 的 Debug = {:?}", w2);

    // ============================================================
    println!("\n6. 文字说明（不可运行/构建侧变更）");
    println!("  - default_alloc_error_handler 稳定：stable 用 alloc 不再要自定义 oom handler");
    println!("  - efiapi 调用约定稳定（UEFI ABI）");
    println!("  - Cargo: crates.io sparse registry 稳定化，配合");
    println!("    [registries.crates-io] protocol = \"sparse\" 大幅加快索引更新");
    println!("  - std::task::Context 改为 !Send + !Sync");
}
