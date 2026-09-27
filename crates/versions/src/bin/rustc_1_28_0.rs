// rustc 1.28.0 演示 —— 全局分配器 + repr(transparent) + NonZeroU8
// 该版本 introduces:
// 1) Language：`GlobalAlloc` trait + `#[global_allocator]` 稳定，
//    此前换分配器必须调用 allocator_api 之类的 unstable 库。
// 2) `#[repr(transparent)]`：newtype 在 FFI 边界与内部类型布局一致。
// 3) `pure/sizeof/alignof/offsetof` 解除保留；#[test] 可返回 Result（示意讲解）。
// 4) Stabilized APIs：NonZero*、step_by、Duration Debug 新格式、Entry::or_default。

use std::alloc::System;

// 1. 全局分配器：整程序改用 std 自带的 System 分配器
#[global_allocator]
static GLOBAL_ALLOCATOR: System = System;

fn main() {
    println!("rustc 1.28.0 演示");

    // 1. 全局分配器（上面的 #[global_allocator] 已生效，这里只是分配内存验证）
    println!("\n1. #[global_allocator] + alloc::System");
    let v: Vec<i32> = (0..3).collect();
    println!("分配器已切换为 System，分配 Vec = {:?}", v);

    // 2. #[repr(transparent)]：newtype 与内部类型 ABI 相同
    println!("\n2. #[repr(transparent)]");
    #[repr(transparent)]
    struct Meters(f64);
    println!(
        "size_of::<Meters>() = {}（= f64，布局一致）",
        std::mem::size_of::<Meters>()
    );

    // 3. 解除保留的关键字
    println!("\n3. pure/sizeof/alignof/offsetof 不再保留");
    let pure = 1;
    let sizeof = 2;
    let alignof = 3;
    let offsetof = 4;
    println!("pure={} sizeof={} alignof={} offsetof={}", pure, sizeof, alignof, offsetof);

    // 4. NonZeroU8：编译期保证非零的类型（占位布局优化）
    println!("\n4. num::NonZeroU8");
    let nz = std::num::NonZeroU8::new(7);
    println!("NonZeroU8::new(7) = {:?}", nz);
    println!("NonZeroU8::new(0) = {:?}", std::num::NonZeroU8::new(0));

    // 5. Iterator::step_by
    println!("\n5. Iterator::step_by");
    let stepped: Vec<i32> = (0..10).step_by(3).collect();
    println!("(0..10).step_by(3) = {:?}", stepped);

    // 6. Duration Debug 新格式
    println!("\n6. Duration Debug 变化");
    println!("{:?}", std::time::Duration::from_secs(1)); // 1s（此前 Duration { secs: 1, nanos: 0 }）

    // 7. Entry::or_default
    println!("\n7. Entry::or_default");
    let mut m: std::collections::HashMap<&str, Vec<i32>> = std::collections::HashMap::new();
    m.entry("k").or_default().push(1);
    println!("entry(\"k\").or_default().push(1) -> {:?}", m);

    // 提示：#[test] 函数可返回 Result —— 在 fn main 中无法直接演示，机制为：
    // test 失败（Err）会以 Debug 打印并计为失败。此处 println 讲解。
    println!("\n8. #[test] 可返回 Result：Ok(()) 通过，Err(e) 计为测试失败（讲解）");
}
