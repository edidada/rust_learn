// rustc 1.43.0 演示 —— 关联常量直用 / once_with / LOG 常量 / Once::is_completed / primitive 模块
use std::sync::Once;

fn main() {
    println!("rustc 1.43.0 演示");
    // 1.43.0 引入：浮点/整型的关联常量可以走类型路径直接用，
    // 不必再 `use std::u32; `、`std::u32::MAX`，而是 `u32::MAX` / `f32::NAN`。
    println!("\n1. 关联常量直接用（u32::MAX / f32::NAN）");
    println!("u32::MAX (旧: std::u32::MAX) = {}", u32::MAX);
    println!("u8::MIN  = {}", u8::MIN);
    println!("f32::NAN 是否 NaN = {}", f32::NAN.is_nan());
    println!("f64::INFINITY + 1.0 = {}", f64::INFINITY + 1.0);

    // 1.43.0 稳定 iter::once_with —— 惰性的单元素迭代器（元素由闭包产生一次）。
    println!("\n2. iter::once_with");
    let mut counter = 0;
    let v: Vec<i32> = std::iter::once_with(|| {
        counter += 1;
        counter * 100
    }).collect();
    println!("once_with(|| counter*100) 收集 = {:?}，闭包只被调用一次", v);

    // 1.43.0 稳定 f32::LOG10_2 / f32::LOG2_10（f64 版本同）。
    println!("\n3. LOG10_2 / LOG2_10");
    println!("f64::LOG10_2 = {}", f64::LOG10_2);
    println!("f64::LOG2_10 = {}", f64::LOG2_10);
    println!("f32::LOG10_2 = {}", f32::LOG10_2);

    // 1.43.0 稳定 Once::is_completed —— 不阻塞、只查询初始化是否已完成。
    println!("\n4. Once::is_completed");
    static INIT: Once = Once::new();
    INIT.call_once(|| println!("~~ 首次初始化发生（call_once 回调）"));
    println!("is_completed() = {}（初始化已发生）", INIT.is_completed());
    let second = Once::new();
    println!("未初始化的 Once：is_completed() = {}", second.is_completed());

    // 1.43.0 引入：std::primitive 模块 re-export 原始类型，宏里避免名字被遮蔽。
    println!("\n5. std::primitive 防遮蔽");
    // 局部 struct 名与原始类型同名也不会搞混：显式走 std::primitive 路径
    struct u8; // 故意遮蔽（本块内 u32 等自动避让）
    impl u8 { fn hi(&self) -> &'static str { "shadow struct" } }
    let s = u8; // 这里是本地 struct 单元
    println!("本地同名 struct: {}", s.hi());
    println!("std::primitive::u32::MAX 不受遮蔽影响 = {}", std::primitive::u32::MAX);
}
