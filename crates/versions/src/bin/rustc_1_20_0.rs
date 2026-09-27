// rustc 1.20.0 演示 —— 关联常量稳定 + f32/f64 位级转换 + 不稳定排序等
// 该版本 introduces:
// 1) Language：关联常量（impl 块里的 const 首次稳定），此前只能定义全局常量或关联函数。
// 2) 大量 Stabilized APIs：f32::from_bits、slice::sort_unstable、Option::get_or_insert 等。
// 3) unimplemented!() 终于可以带消息了（旧行为：只能无参调用）。

fn main() {
    println!("rustc 1.20.0 演示");

    // 1. 关联常量：1.20 起可在 impl 块中定义 const（对比旧行为：只能写全局常量）
    println!("\n1. 关联常量 stable");
    struct Level;
    impl Level {
        const MAX: u8 = 10;
        const MIN: u8 = 0;
    }
    println!("Level::MIN={}, Level::MAX={}", Level::MIN, Level::MAX);

    // 2. f32::from_bits / to_bits：位模式与浮点数互换（1.20 稳定）
    println!("\n2. f32 位级转换");
    let bits: u32 = 0x3F80_0000; // 1.0f32 的位模式
    let f: f32 = f32::from_bits(bits);
    println!("0x3F800000 -> {}; 回转位: 0x{:X}", f, f.to_bits().to_be());

    // 3. slice::sort_unstable：不稳定的原地排序，比 sort 快且省内存
    println!("\n3. sort_unstable");
    let mut v = [3usize, 1, 4, 1, 5];
    v.sort_unstable();
    println!("排序后: {:?}", v);

    // 4. Option::get_or_insert(_with)：为空则插入默认值
    println!("\n4. Option::get_or_insert");
    let mut opt: Option<i32> = None;
    let got = opt.get_or_insert(42);
    *got += 1;
    println!("{:?}", opt); // Some(43)
    let mut opt2: Option<i32> = None;
    let v2 = opt2.get_or_insert_with(|| 7);
    println!("get_or_insert_with -> {}", v2);

    // 5. mem::ManuallyDrop：手动控制析构的包装类型（1.20 稳定）
    println!("\n5. mem::ManuallyDrop");
    let m = std::mem::ManuallyDrop::new(String::from("不自动析构"));
    // 说明：ManuallyDrop 让 SW 不在离开作用域时 Drop；
    // 如需取出并释放，需要 unsafe { ManuallyDrop::drop(..) } 或后续版本的 into_inner。
    println!("通过 Deref 读取: {}，size_of::<String>()={}", &*m, std::mem::size_of::<String>());

    // 6. str::get：安全（返回 Option）的范围访问字节切片视角
    println!("\n6. str::get");
    let s = "hello";
    println!("s.get(1..4) = {:?}", s.get(1..4));

    // 7. unimplemented! 带消息 / compile_error!：历史能力，直接讲解不触发
    println!("\n7. 宏增强（只讲解，不实际 panic/编译失败）");
    // 1.20 起 unimplemented! 可以带消息：unimplemented!("等 1.21 稳定");
    // compile_error!("...") 在被宏展开且真正执行时触发编译错误，常用于版本/配置守卫。
    println!("unimplemented!(\"msg\") 与 compile_error!(\"msg\") 已稳定，调用即 panic / 报编译错");
    println!("char::escape_debug 示例: {:?}", 'a'.escape_debug());
}
