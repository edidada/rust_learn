// rustc 1.81.0 演示 —— #[expect] lint 预期、fs::exists、Duration::abs_diff、IoSlice、PanicHookInfo
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.81 稳定的 API 在 1.98 均可用。
use std::io::IoSlice;

// 1.81 稳定 #[expect]：与 allow 相反，"预期这段代码会触发某 lint"。
// 若 lint 真的发生了 → 静默；若没有发生 → unfulfilled_lint_expectations 警告提醒你清理。
// 下面这个函数没有任何调用者，会触发 dead_code，#[expect] 正好满足，不产生任何告警。
#[expect(dead_code)]
fn placeholder_unused() -> u32 {
    0
}

fn main() {
    println!("rustc 1.81.0 演示");

    println!("\n1. #[expect] lint 预期（1.81 重点）");
    // 对比三种写法：
    //   #[allow(dead_code)]  → 死代码永远静默，即使后来有人开始调用（lint 消失）也无人察觉
    //   #[expect(dead_code)] → 死代码被"预期"；一旦它不再死代码，编译器警告"预期未满足"
    //   （什么都不写）       → 直接得到 dead_code 警告
    println!("placeholder_unused 被 #[expect(dead_code)] 静默（真实存在）");
    // 展示"预期未满足"的机制：一个肯定会运行的代码块预期 an unused lint，会得到警告。
    // 演示里用 allowed lint 避免仓库告警，机制相同：
    let _ = std::hint::black_box(1);
    println!("（机制讲解：expect 满足=静默，不满足=unfulfilled_lint_expectations 警告）");

    println!("\n2. fs::exists —— 只检查存在性，不打开文件");
    // 旧写法：path.exists() / fs::metadata() 都可能因权限错误而失败或误判；
    // fs::exists 返回 Result<bool>，语义专一：能定位到条目即 true。
    println!("Cargo.toml 存在? {:?}", std::fs::exists("Cargo.toml"));
    println!("不存在文件? {:?}", std::fs::exists("definitely-not-here-xyz"));

    println!("\n3. Duration::abs_diff（1.81 稳定）");
    let d1 = std::time::Duration::from_secs(30);
    let d2 = std::time::Duration::from_secs(50);
    println!("abs_diff(30s, 50s) = {:?}（无符号差，无需担心负数）", d1.abs_diff(d2));

    println!("\n4. IoSlice::advance / advance_slices");
    let buf1 = b"hello".as_slice();
    let buf2 = b"world".as_slice();
    let mut slices = [IoSlice::new(buf1), IoSlice::new(buf2)];
    let mut slices: &mut [IoSlice] = &mut slices;
    IoSlice::advance_slices(&mut slices, 3); // 跳过 "hel"
    let rest: Vec<u8> = slices.iter().flat_map(|s| s.as_ref().iter().copied()).collect();
    println!("advance_slices(3) 后剩余 = {}", String::from_utf8_lossy(&rest));

    println!("\n5. PanicHookInfo / core::error 讲解");
    // 1.81 拆分 panic 类型：std 的 hook 参数改叫 std::panic::PanicHookInfo，
    // core 的 #[panic_handler] 参数仍是 core::panic::PanicInfo（no_std）。
    // 这里用 std hook 真实走一遍：
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo| {
        println!("  （hook 收到 PanicHookInfo，payload = {:?}）", info.payload_as_str());
    }));
    let r = std::panic::catch_unwind(|| panic!("boom-81"));
    std::panic::set_hook(default_hook);
    println!("catch_unwind 结果 = {:?}（hook 名字即 1.81 的改名点）", r.is_err());
    // core::error：no_std 下也能用 Error trait（1.81 把它从 std 提升到 core）。
    let e: Box<dyn std::error::Error + Send + Sync> = Box::new(std::io::Error::other("core::error"));
    println!("Error trait 可用于 core（演示 Box<dyn Error>）= {e}");
}
