// rustc 1.60.0 演示 —— Instant 饱和、abs_diff、MaybeUninit::assume_init_read
fn main() {
    println!("rustc 1.60.0 演示");

    // ============================================================
    println!("\n1. Instant::duration_since/elapsed/sub 饱和而非 panic");
    // 1.60 起：结果为负（时钟回滑/倒序调用）时得 0，不再 panic。
    // 对比旧行为：duration_since(未来时间) 会 panic。
    use std::time::{Duration, Instant};
    let later = Instant::now();
    let earlier = later - Duration::from_secs(1);
    let d = later.duration_since(earlier);
    println!("  正常序 duration_since = {:?}", d);
    let d2 = later.saturating_duration_since(earlier); // 饱和变体 1.60 前后都有
    if d2 < Duration::from_secs(2) {
        println!("  saturating_duration_since 取值 = {:?}（不会 panic）", d2);
    }

    // ============================================================
    println!("\n2. 整数 abs_diff（1.60 稳定，全宽）");
    println!("  3u32.abs_diff(9) = {}", 3u32.abs_diff(9));
    println!("  i8 类型也支持：(-5i8).abs_diff(2) = {}", (-5i8).abs_diff(2));
    println!("  usize::abs_diff(3, 10) = {}", 3usize.abs_diff(10));

    // ============================================================
    println!("\n3. HashMap::from + ExitCode 进程退出码");
    use std::collections::HashMap;
    let m: HashMap<char, i32> = HashMap::from([('a', 97), ('b', 98)]);
    println!("  HashMap::from([('a',97)]) 已可用，a = {}", m[&'a']);
    // From<u8> for ExitCode：把字节数变成进程退出码（1.60 稳定）
    use std::process::ExitCode;
    let code = ExitCode::from(7u8);
    println!("  ExitCode::from(7u8) = {:?}", code);
    // （未真退出程序；仅演示构造。真实 main 返回它时进程 exit code = 7）

    // ============================================================
    println!("\n4. MaybeUninit::assume_init_read / assume_init_drop");
    use std::mem::MaybeUninit;
    let mu: MaybeUninit<u64> = MaybeUninit::new(12345u64);
    // assume_init_read: 拷贝出初始化值（按值读）——避免 move out 语义
    let v: u64 = unsafe { mu.assume_init_read() };
    println!("  assume_init_read() = {}", v);
    // Vec::spare_capacity_mut：拿到未初始化余量
    let mut vec: Vec<u8> = Vec::with_capacity(4);
    let spare: &mut [std::mem::MaybeUninit<u8>] = vec.spare_capacity_mut();
    spare[0].write(9);
    unsafe { vec.set_len(1) }; // 仅演示可变余量手工初始化
    println!("  手工写 spare_capacity_mut 后 vec = {:?}", vec);

    // ============================================================
    println!("\n5. #[cfg(panic = ...)] / #[cfg(target_has_atomic = ...)] 讲解");
    println!("  - cfg(panic = \"abort\") / cfg(panic = \"unwind\") 可按编译选择分支。");
    println!("  - cfg(target_has_atomic = \"64\") 判断该目标是否支持 64 位原子。");
    println!("  - 本演示在默认配置下两分支均不会突变，仅说明能力。");
}
