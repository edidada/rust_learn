// rustc 1.31.0 演示 —— Edition 2018 模块路径变化 + const fn
// 该版本 introduces:
// 1) 🎉 Edition 2018：use 语句默认从 crate 根/外部 crate 解析，不再先找当前模块。
//    引用 crate 根必须写 `crate::foo`；外部 crate 直接写名字，无需 extern crate。
//    （本文件是 2024 edition 编译的，语义一致：crate:: 与顶层 use 解析规则相同。）
// 2) 新生命周期省略规则：impl 头可用 BufReader<'_> 代替 BufReader<'a>。
// 3) const fn 稳定。
// 4) Stabilized APIs：Option::replace、slice::chunks_exact/rchunks、NonZero* From。

// —— 1. 模块路径（2018）：use 直接从 crate 根解析 ——
mod engine {
    // 2018 前的写法（1.31 前）：use ::start; 或 use super::start;
    // 2018 起：use crate::start; —— 明确指 crate 根
    use crate::start;
    pub fn run() {
        start();
    }
}

fn start() {
    println!("crate::start() 被模块内 use crate::start 调用");
}

// —— 2. impl 头的 '_（新省略规则）——
struct Buffer<'a> {
    data: &'a [u8],
}
impl Buffer<'_> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

// —— 3. const fn ——
const fn square(x: i32) -> i32 {
    x * x
}
const SQUARE_OF_5: i32 = square(5); // 常量上下文中调用

fn main() {
    println!("rustc 1.31.0 演示");

    // 1. 2018 edition 模块路径
    println!("\n1. Edition 2018：use crate:: 与直接外部 crate 名");
    engine::run();

    // 2. impl Buffer<'_> 里的 '_
    println!("\n2. 生命周期省略：impl Buffer<'_>");
    let b = Buffer { data: b"hello" };
    println!("Buffer::len = {}", b.len());

    // 3. const fn
    println!("\n3. const fn");
    println!("square(5) const 求值 = {}", SQUARE_OF_5);
    println!("运行时调用 square(7) = {}", square(7));

    // 4. Option::replace
    println!("\n4. Option::replace");
    let mut o = Some("旧值");
    let old = o.replace("新值");
    println!("old = {:?}, o = {:?}", old, o);

    // 5. chunks_exact / rchunks
    println!("\n5. slice::chunks_exact / rchunks");
    let data = [1, 2, 3, 4, 5];
    println!("chunks_exact(2) = {:?}", data.chunks_exact(2).collect::<Vec<_>>());
    println!("rchunks(2)       = {:?}", data.rchunks(2).collect::<Vec<_>>());

    // 6. NonZero -> 原始类型 From
    println!("\n6. From<NonZeroU8> for u8");
    let nz = std::num::NonZeroU8::new(9).unwrap();
    let raw: u8 = nz.into();
    println!("u8::from(NonZeroU8(9)) = {}", raw);
}
