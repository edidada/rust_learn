// rustc 1.23.0 演示 —— auto trait 组合 + AsciiExt 弃用 + AtomicPtr From
// 该版本 introduces:
// 1) Language：trait object 允许更多 auto trait 组合（dyn Trait + Send + SendSync 等）。
// 2) 弃用 AsciiExt：ASCII 方法变成基本类型固有方法；旧代码需要 `use std::ascii::AsciiExt`，1.23 起不需要。
// 3) Atomic* 实现 From：From<usize/isize> for AtomicUsize/AtomicIsize、From<*mut T> for AtomicPtr<T>。

fn main() {
    println!("rustc 1.23.0 演示");

    // 1. auto trait 组合：dyn Trait + Send + Sync（auto trait 集合更自由）
    println!("\n1. dyn Trait + auto trait 组合");
    trait Speaker {
        fn speak(&self) -> String;
    }
    struct Dog;
    impl Speaker for Dog {
        fn speak(&self) -> String {
            "汪".to_string()
        }
    }
    // 1.23 起 auto trait 可任意组合（ historically 仅 Send/Sync 等核心少数）
    let s: Box<dyn Speaker + Send + Sync> = Box::new(Dog);
    println!("多 auto trait 盒子: {}", s.speak());

    // 2. AsciiExt 迁移：固有方法直接可用
    println!("\n2. AsciiExt 弃用，ASCII 方法成为固有/内建方法");
    // 旧写法（已弃用，仅示意）：
    //   use std::ascii::AsciiExt;
    //   "rust".to_ascii_uppercase()
    // 1.23 起基本类型自带这些方法：
    println!("\"rust\".to_ascii_uppercase() = {}", "rust".to_ascii_uppercase());

    // 3. Atomic* From 转换
    println!("\n3. Atomic* 的 From 转换");
    let a: std::sync::atomic::AtomicUsize = 5usize.into();
    let p: std::sync::atomic::AtomicPtr<u8> =
        Box::into_raw(Box::new(7u8)).into(); // From<*mut T> → AtomicPtr
    println!("AtomicUsize={:?}, *原子内部的裸指针指针尺寸={}", a, std::mem::size_of_val(&p));
    // 归还所有权避免泄漏（swap 后释放）
    let raw = std::sync::atomic::AtomicPtr::new(p.into_inner().cast::<u8>().cast::<u8>());
    unsafe { drop(Box::from_raw(raw.into_inner())) };

    // 4. assert_eq!/assert_ne! 允许尾逗号
    println!("\n4. assert_eq! 尾逗号");
    assert_eq!(1 + 1, 2, "现在允许尾部逗号",);
    println!("assert_eq!(2,2,\"msg\",) 编译通过");
}
