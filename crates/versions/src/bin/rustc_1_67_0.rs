// rustc 1.67.0 演示 —— ilog 家族、const char、ControlFlow、舍入与 drop 顺序
fn main() {
    println!("rustc 1.67.0 演示");

    // ============================================================
    println!("\n1. ilog / checked_ilog / NonZero 系列");
    // 1.67 稳定整数的对数 API
    println!("  100u32.ilog10() = {}", 100u32.ilog10()); // 2
    println!("  8u32.ilog2() = {}", 8u32.ilog2()); // 3
    println!("  123456u32.ilog(10) = {}", 123456u32.ilog(10)); // 5
    // checked 版本：输入 0 或负数返回 None 而不是 panic
    println!("  0i32.checked_ilog2() = {:?}", 0i32.checked_ilog2()); // None
    println!("  (-4i32).checked_ilog2() = {:?}", (-4i32).checked_ilog2()); // None
    println!("  256u32.checked_ilog2() = {:?}", 256u32.checked_ilog2()); // Some(8)
    // NonZeroU*::ilog2/ilog10 与 NonZero*::BITS
    let nz = std::num::NonZeroU32::new(1024).unwrap();
    println!("  NonZeroU32(1024).ilog2() = {}", nz.ilog2()); // 10
    println!("  NonZeroU32::BITS = {}", std::num::NonZeroU32::BITS); // 32

    // ============================================================
    println!("\n2. const 上下文中的 char::from_u32 / from_digit / to_digit");
    // 1.67 起 char 的这几个 API 可用于 const
    const A: char = char::from_u32(0x41).unwrap(); // 'A'
    const DIGIT_CH: char = char::from_digit(9, 10).unwrap();
    println!("  const char::from_u32(0x41) = {}", A);
    println!("  const char::from_digit(9,10) = {}", DIGIT_CH);
    println!("  'a'.to_digit(16) = {:?}", 'a'.to_digit(16)); // Some(10)

    // ============================================================
    println!("\n3. ControlFlow derive Eq / Hash");
    use std::ops::ControlFlow;
    let a: ControlFlow<u8, i32> = ControlFlow::Break(3);
    let b: ControlFlow<u8, i32> = ControlFlow::Break(3);
    println!("  Break(3) == Break(3) -> {}", a == b);
    let mut set = std::collections::HashSet::new();
    set.insert(ControlFlow::<u8, i32>::Continue(1));
    set.insert(ControlFlow::<u8, i32>::Break(2));
    println!("  HashSet<ControlFlow> 长度 = {}（可用作键）", set.len());

    // ============================================================
    println!("\n4. 0.5 格式化为 0 位小数：ties to even");
    // 1.67 修复：0.5 舍入到 0（与浮点格式化其它地方的 ties-to-even 一致）
    println!("  format!(\"{{:.0}}\", 0.5) = \"{}\"", format!("{:.0}", 0.5)); // "0"
    println!("  format!(\"{{:.0}}\", 1.5) = \"{}\"", format!("{:.0}", 1.5)); // "2"

    // ============================================================
    println!("\n5. && / || 链的临时值 drop 顺序（left-to-right）");
    struct D(&'static str);
    impl D {
        fn t(&self) -> bool {
            true
        }
    }
    impl Drop for D {
        fn drop(&mut self) {
            println!("  drop {}", self.0);
        }
    }
    // 1.67 起临时值按 A、B、C 的书写顺序（从左到右）drop；
    // 以前"扭曲"：第一个表达式的临时值反而最后 drop。
    let r = D("A").t() && D("B").t() && D("C").t();
    println!("  链结果 = {}", r);

    // ============================================================
    println!("\n6. 行为说明（文字）");
    println!("  - std::sync::mpsc 内部实现换为 crossbeam-channel，性能更佳");
    println!("  - Sized 谓词 coinductive：允许 trait bound 的循环而不再误报");
    println!("  - 字符串字面量下划线后缀成为硬错误：\"abc\"_u32 之类不再合法");
}
