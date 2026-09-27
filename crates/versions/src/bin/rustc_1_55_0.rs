// rustc 1.55.0 演示 —— 开放范围模式、array::map、ControlFlow、MaybeUninit::write
fn main() {
    println!("rustc 1.55.0 演示");

    // ============================================================
    println!("\n1. 开放起点范围模式 X..");
    // 1.55 稳定：X.. 作为模式匹配“从 X 到整型最大值”。
    // 对比旧行为：需要写成 X ..= u32::MAX。
    for n in [3, 99, 100, 101u32] {
        match n {
            0..=99 => println!("  {} 在 0..=99", n),
            100.. => println!("  {} 在 100..（到 u32 最大）", n),
        }
    }

    // ============================================================
    println!("\n2. array::map —— 数组逐元素映射");
    // 1.55 稳定 [T; N]::map：在栈上完成转换，不产生堆分配。
    let bytes = [1u8, 2, 3];
    let words = bytes.map(|b| b as u32 * 100);
    println!("  [1,2,3].map(*100) = {:?}", words);

    // ============================================================
    println!("\n3. ops::ControlFlow");
    use std::ops::ControlFlow;
    // ControlFlow<B, C>：Break(B) / Continue(())，用于遍历类 API 的“提前退出”。
    let cf: ControlFlow<&str> = ControlFlow::Break("stop");
    println!("  ControlFlow::is_break = {}", cf.is_break());
    let cf2: ControlFlow<&str> = ControlFlow::Continue(());
    println!("  Continue::is_continue = {}", cf2.is_continue());

    // ============================================================
    println!("\n4. MaybeUninit::write（注意配套 assume_init_* 系列也稳定）");
    use std::mem::MaybeUninit;
    let mut slot: MaybeUninit<u32> = MaybeUninit::uninit();
    // write：把值放入并使该 MaybeUninit 处于已初始化状态
    let p = slot.write(42);
    println!("  MaybeUninit::write(42) 后读 = {}", unsafe { *p });
    // assume_init_ref / assume_init_mut：对已初始化内存取引用（本例通过 write 保证初始化）
    let rf: &[u32] = unsafe { slot.assume_init_ref() };
    println!("  assume_init_ref()[0] = {}", rf[0]);

    // ============================================================
    println!("\n5. 兼容性提醒（println 讲解）");
    println!("  - std 返回 io::Error 时不再一律用 ErrorKind::Other");
    println!("  - Windows 上 Command 的环境变量名不再 ASCII 大写化");
    println!("  - RUSTFLAGS 不再给 build 脚本；用 CARGO_ENCODED_RUSTFLAGS");
}
