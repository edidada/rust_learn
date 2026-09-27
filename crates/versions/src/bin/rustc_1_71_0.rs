// rustc 1.71.0 演示 —— hash_one、thread_local const {}、NonZero 取负、数组元组转换
fn main() {
    println!("rustc 1.71.0 演示");

    // ============================================================
    println!("\n1. BuildHasher::hash_one");
    use std::hash::BuildHasher;
    use std::collections::hash_map::RandomState;
    // 以前要 new 一个 Hasher、write、finish；现在一行搞定
    let h: u64 = RandomState::new().hash_one("rust 1.71");
    println!("  RandomState::new().hash_one(\"rust 1.71\") = {}", h);
    let h2: u64 = RandomState::new().hash_one(1234_i32);
    println!("  hash_one(1234) = {}", h2);

    // ============================================================
    println!("\n2. std::thread_local 的 const {{}} 语法");
    use std::cell::Cell;
    // 1.59 稳定、1.71 文档化：const {} 让线程局部变量零运行时初始化
    thread_local! {
        static COUNTER: Cell<i32> = const { Cell::new(0) };
    }
    COUNTER.with(|c| {
        c.set(c.get() + 1);
        println!("  线程局部 COUNTER = {}", c.get());
    });

    // ============================================================
    println!("\n3. NonZeroI* 符号判断与取负");
    use std::num::NonZeroI32;
    let nz = NonZeroI32::new(-5).unwrap();
    println!("  NonZeroI32(-5).is_negative() = {}", nz.is_negative());
    println!("  NonZeroI32(5).is_positive() = {}", NonZeroI32::new(5).unwrap().is_positive());
    println!("  -nz（Neg for NonZero）= {}", -nz);
    println!("  checked_neg(-5) = {:?}", nz.checked_neg()); // Some(5)
    let z = NonZeroI32::new(0).unwrap_or(NonZeroI32::MIN); // 0 不能构造 NonZero
    println!("  0 无法构造 NonZero（演示用 MIN 替代）= {}", z);

    // ============================================================
    println!("\n4. 数组 ⇄ 元组 From 转换（N ≤ 12）");
    let arr: [i32; 3] = [1, 2, 3];
    let tup: (i32, i32, i32) = arr.into(); // From<[T;N]> for tuple
    println!("  [1,2,3] -> tuple = {:?}", tup);
    let back: [i32; 3] = tup.into(); // From<tuple> for [T;N]
    println!("  tuple -> [1,2,3] = {:?}", back);

    // ============================================================
    println!("\n5. CStr::is_empty");
    let empty = std::ffi::CStr::from_bytes_with_nul(b"\0").unwrap();
    let not_empty = std::ffi::CStr::from_bytes_with_nul(b"a\0").unwrap();
    println!("  C\"\\0\".is_empty() = {}, C\"a\\0\".is_empty() = {}",
        empty.is_empty(), not_empty.is_empty());

    // ============================================================
    println!("\n6. const 上下文 read / split_at");
    // 1.71 起 ptr::read、slice::split_at 可在 const 中使用
    const fn head2(s: &[i32]) -> [i32; 2] {
        let (a, _) = s.split_at(2); // const split_at
        [a[0], a[1]]
    }
    const HEAD: [i32; 2] = head2(&[10, 20, 30]);
    println!("  const split_at 取头部 = {:?}", HEAD);
    // ptr::read 的 const 形式：const COPY: i32 = unsafe { std::ptr::read(&5) };（此处展示思路）

    // ============================================================
    println!("\n7. 文字说明");
    println!("  - extern \"C-unwind\" 稳定：明确跨语言 unwind 的 ABI 语义");
    println!("  - raw-dylib/link_ordinal 稳定：Windows 链接无需导入库、可指定序号");
    println!("  - TypeId 不再 structural match：不能在模式中用常量 TypeId");
}
