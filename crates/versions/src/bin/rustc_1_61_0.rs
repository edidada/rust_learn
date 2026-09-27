// rustc 1.61.0 演示 —— const fn 泛型 bound、stdio lock 'static、ExitCode、retain_mut
fn main() -> std::process::ExitCode {
    println!("rustc 1.61.0 演示");

    // ============================================================
    println!("\n1. const fn 能力扩展（println + 受限演示）");
    // 1.61 起 const fn 可以：
    //   (a) 依赖泛型 trait bound；(b) 用 impl Trait 参数/返回值；
    //   (c) 创建/cast/传递函数指针；(d) 递归调用设置 opaque 返回值。
    // (a) 泛型 bound 的 const fn：泛型参数直接参与 const 计算。
    //     （const 上下文内 trait 方法调用仍受限，教学用整型取模表达。）
    const fn modulus<const N: usize>(x: usize) -> usize {
        x % N
    }
    const M: usize = modulus::<5>(12); // 12 % 5 = 2
    println!("  modulus::<5>(12) = {}（const 泛型参数在 const fn 中工作）", M);
    // (b) 函数指针在 const fn / const 上下文里创建：1.61 允许 const 里存函数指针，
    //     但"调用函数指针"当时仍是 E0015（后续版本才放开），这里只演示创建与打印：
    const FN_PTR: fn(u32) -> u32 = double;
    let runtime = FN_PTR(21);
    println!("  const 定义的函数指针（运行期调用 double(21)）= {}", runtime);
    // (c) impl Trait 参数/返回位置、递归设置 opaque 返回值（println 讲解）：
    println!("  impl Trait 参数/返回位置的 const fn、递归设置 opaque 返回值也已允许。");

    // ============================================================
    println!("\n2. std::io::stdout().lock() 得到 'static 句柄");
    // 1.61 起 lock() 返回 'static 句柄，之前不能直接写的这行现在可以：
    let mut handle = std::io::stdout().lock();
    // 对比旧行为：lock() 借用 stdout() 临时值，须先 let out = std::io::stdout(); 再 out.lock()。
    use std::io::Write;
    let _ = writeln!(handle, "  锁定的 stdout 句柄可写入");
    let _ = writeln!(handle, "  （1.61 前 out 借用问题已修复）");

    // ============================================================
    println!("\n3. std::process::ExitCode / Termination");
    // 1.61 稳定：main 可返回实现 Termination 的类型，如 ExitCode，实现自定义退出码。
    // 本演示在程序尾部返回 Failed(=1) 之外的其它值（实际是 SUCCESS，见 return）。
    println!("  main 将返回 ExitCode::SUCCESS（即 0），演示 Termination trait.");

    // ============================================================
    println!("\n4. Vec::retain_mut / VecDeque::retain_mut");
    let mut v = vec![1, 2, 3, 4];
    v.retain_mut(|x| {
        *x *= 2; // 可变访问：保留者也被原位修改
        *x > 3
    });
    println!("  retain_mut(*2, >3) = {:?}", v);
    use std::collections::VecDeque;
    let mut dq: VecDeque<i32> = [1, 2, 3, 4].into_iter().collect();
    dq.retain_mut(|x| match x {
        x if *x % 2 == 0 => ok_keep(x),
        _ => false,
    });
    println!("  VecDeque retain_mut 保留偶数 = {:?}", dq);

    // ============================================================
    println!("\n5. 兼容性提醒");
    println!("  - cfg(all())/cfg(any()) 不再短路，全部谓词都会求值。");
    println!("  - dyn 类型 trait bound 检查会强制超级 trait bound。");
    println!("  - native static 链接不再默认 whole-archive，链接错误需重排依赖。");
    println!("  - JoinHandle::is_finished 也在本版稳定（非阻塞查询线程状态）。");

    std::process::ExitCode::SUCCESS
}

fn double(x: u32) -> u32 {
    x * 2
}

fn ok_keep(x: &mut i32) -> bool {
    *x -= 1;
    true
}
