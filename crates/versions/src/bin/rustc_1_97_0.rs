// rustc 1.97.0 演示 —— isolate_highest/lowest_one、highest/lowest_one、bit_width、RepeatN
// 注意：本仓库以 rustc 1.98 运行；1.97 稳定的 API 在 1.98 均可用。
use std::iter::repeat_n;
use std::num::NonZeroU32;

fn main() {
    println!("rustc 1.97.0 演示");

    println!("\n1. 整数 isolate_highest_one / isolate_lowest_one（1.97 稳定）");
    // isolate：把"最高/最低的 1 位"单独留下，其余清零 —— 旧写法要 trailing_zeros/pow 手拼
    let n: u32 = 0b0101_1000;
    println!("0b01011000.isolate_highest_one() = {:#b}", n.isolate_highest_one()); // 0b01000000
    println!("0b01011000.isolate_lowest_one()  = {:#b}", n.isolate_lowest_one()); // 0b1000

    println!("\n2. highest_one / lowest_one / bit_width（1.97 稳定）");
    println!("0b01011000.highest_one() = {:?}", n.highest_one()); // Some(6)
    println!("0b01011000.lowest_one()  = {:?}", n.lowest_one()); // Some(3)
    println!("0b01011000.bit_width()   = {}", n.bit_width()); // 7
    println!("0u32.highest_one() = {:?}（None）", 0u32.highest_one());

    println!("\n3. NonZero 同名五件套（1.97 稳定：保证非零，方法直接返回非 Option/NonZero）");
    let nz = NonZeroU32::new(0b0101_1000).unwrap();
    println!("nz.isolate_highest_one() = {:#b}", nz.isolate_highest_one());
    println!("nz.lowest_one()          = {}", nz.lowest_one()); // NonZero 返回值直接是下标，不是 Option
    println!("nz.highest_one()         = {}", nz.highest_one());
    println!("nz.bit_width()           = {}", nz.bit_width());

    println!("\n4. RepeatN 实现 Default（1.97 稳定）");
    // repeat_n(值, n)：克隆式重复；Default 给出"零重复"的空迭代器
    let r = repeat_n("x", 3);
    println!("repeat_n(\"x\", 3) = {:?}", r.collect::<Vec<&str>>());
    let empty: std::iter::RepeatN<i32> = Default::default();
    println!("RepeatN::default() 空 = {:?}", empty.collect::<Vec<i32>>());

    println!("\n5. 其余 1.97 兼容性条目（记叙）");
    // pin! 的 deref 强转移除：pin!(x)（x: &mut T）恒为 Pin<&mut &mut T>
    let mut v = 5u8;
    let pinned = std::pin::pin!(&mut v); // Pin<&mut &mut u8>
    println!("pin!(&mut x) 的类型恒为 Pin<&mut &mut T>（不再被强转成 Pin<&mut T>）");
    println!("Result<T, Infallible>/ControlFlow<!, T> 在 must_use 中视同 T（不可能的错误免查）");
    println!("v0 符号重整默认启用：老版调试器可能无法 demangle");
    println!("弃用 std::char 常量/函数，改用 char:: 路径");
    let _ = pinned;
}
