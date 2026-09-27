// rustc 1.37.0 演示 —— 类型别名枚举变体 / const _ / xor / reverse_bits / copy_within
use std::io::BufReader;

fn main() {
    println!("rustc 1.37.0 演示");
    // 1.37.0 引入：类型别名现在可以直接引用枚举变体，
    // `MyOption::Some(y)` 这样的路径在 1.37 之前写不出来（会提示找不到变体）。
    println!("\n1. 类型别名引用枚举变体");
    type MyOption = Option<u8>;
    fn increment_or_zero(x: MyOption) -> u8 {
        match x {
            MyOption::Some(y) => y + 1,
            MyOption::None => 0,
        }
    }
    println!("increment_or_zero(Some(3)) = {}", increment_or_zero(MyOption::Some(3)));
    println!("increment_or_zero(None)    = {}", increment_or_zero(MyOption::None));

    // 1.37.0 引入：`_` 可以作为 const 的名字，匿名声明的 const 常用于编译期断言。
    println!("\n2. const _ 匿名常量");
    const _INNER_ASSERT: usize = std::mem::size_of::<u32>(); // 旧版这里的标识符必须有名字
    // 编译期断言写法：const _: () = if ... { panic!() };（1.98 的 const fn 已可做更多事，
    // 当时常用 `const _: () = assert!(...)` 或自定义 const fn 断言，例如：
    const _: () = assert!(std::mem::size_of::<u32>() == 4);
    const _: () = assert!(std::mem::size_of::<u32>() == 4); // 故意重复演示 const _ 的"可声明多个"

    // 1.37.0 稳定：Option::xor —— "恰好一个为 Some" 语义（类似 bool 的 XOR）。
    println!("\n3. Option::xor");
    let a: Option<u8> = Some(1);
    let b: Option<u8> = None;
    println!("Some(1).xor(None)            = {:?}", a.xor(b));
    println!("Some(1).xor(Some(2))         = {:?}", Some(1u8).xor(Some(2u8)));
    println!("None::<u8>.xor(None)         = {:?}", None::<u8>.xor(None));

    // 1.37.0 稳定：reverse_bits（各整数类型）、slice::copy_within、DoubleEndedIterator::nth_back。
    println!("\n4. reverse_bits / copy_within / nth_back");
    let x: u8 = 0b1100_0000;
    println!("0b1100_0000.reverse_bits() = {:#010b}", x.reverse_bits()); // 0000_0011
    let mut arr = [1, 2, 3, 4, 5];
    arr.copy_within(1..4, 0); // 1.37 稳定：把 src 范围 memcpy 到 dest 起点
    println!("copy_within(1..4, 0) 后 arr = {:?}", arr);
    let deque: std::collections::VecDeque<i32> = (1..=5).collect();
    println!("VecDeque nth_back(0) = {:?}", deque.iter().nth_back(0)); // 尾部第 1 项
    println!("Wrapping(0b1000_0000u8).reverse_bits() = {:?}",
        std::num::Wrapping(0b1000_0000u8).reverse_bits());

    // 1.37.0 稳定：BufReader::buffer —— 直接观察当前内部缓冲区内容，不用重新读。
    println!("\n5. BufReader::buffer");
    let data: &[u8] = b"sleepy";
    let mut reader = BufReader::with_capacity(8, data);
    use std::io::Read as _;
    let mut first = [0u8; 3];
    reader.read_exact(&mut first).unwrap();
    println!("读了 3 字节后，BufReader::buffer() 剩余缓冲 = {:?}", reader.buffer());
}
