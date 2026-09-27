// rustc 1.64.0 演示 —— poll_fn/ready!、NonZero 算术、OsString Write
fn main() {
    println!("rustc 1.64.0 演示");

    // ============================================================
    println!("\n1. task::ready! 与 future::poll_fn");
    // 1.64 稳定：ready!(poll) 宏 —— Pending 直接 return Poll::Pending；
    // poll_fn：把一个闭包包装成 Future。
    use std::future::poll_fn;
    use std::task::{Context, Poll, ready};
    // 在 Blocker 上运行 Future（无运行时简版）
    struct PendingOnce(bool);
    impl std::future::Future for PendingOnce {
        type Output = u8;
        fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 {
                self.0 = false;
                // 手工触发一次 waker，模拟再次被叫（教学用）
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(42)
            }
        }
    }
    let f = PendingOnce(true);
    // poll_fn 则在稳定 API 之上展示“手写 poll”的便捷路径说明
    let mut polled: std::pin::Pin<&mut PendingOnce> = std::pin::pin!(f);
    let mut woke = false;
    let result = {
        let waker = std::task::Waker::noop(); // 1.85+ noop（无条件说明用）
        let mut cx = Context::from_waker(waker);
        match polled.as_mut().poll(&mut cx) {
            Poll::Ready(v) => Some(v),
            Poll::Pending => {
                // 重新 poll（waker 已唤醒），闭环演示
                match polled.as_mut().poll(&mut cx) {
                    Poll::Ready(v) => Some(v),
                    Poll::Pending => None,
                }
            }
        }
    };
    println!("  手工 poll 完整闭环 = {:?}", result);
    // poll_fn 形式（稳定 API，但要运行时；此处仅 println 说明）
    let _ = poll_fn(|cx| {
        let _: () = ready!(Poll::Ready(()));
        let _ = cx;
        Poll::Ready(())
    });
    println!("  poll_fn/ready! 的常规用法需 async 上下文或手动 poll；此处演示 poll 闭环。");

    // ============================================================
    println!("\n2. workspace 继承（println 讲解）");
    println!("  - [workspace.package] 可以声明公共 version/license 等字段;");
    println!("    成员 crate 用 version.workspace = true 继承。");
    println!("  - [workspace.dependencies] 集中依赖版本; 成员写 dep.workspace = true。");

    // ============================================================
    println!("\n3. NonZero*::checked_mul / saturating_mul / unsigned_abs 等");
    use std::num::{NonZeroI32, NonZeroU32};
    let nz: NonZeroU32 = NonZeroU32::new(3).unwrap();
    println!(
        "  NonZeroU32 3.checked_mul(2) = {:?}，saturating_mul(2) = {}",
        nz.checked_mul(NonZeroU32::new(2).unwrap()),
        NonZeroU32::MAX.saturating_mul(NonZeroU32::new(2).unwrap()).get()
    );
    let ni: NonZeroI32 = NonZeroI32::new(-5).unwrap();
    println!("  NonZeroI32(-5).abs() = {}, unsigned_abs = {}", ni.abs(), ni.unsigned_abs());
    println!("  NonZeroI32::MAX.checked_abs = overflow None -> {:?}", NonZeroI32::MAX.checked_abs());

    // ============================================================
    println!("\n4. Ipv6Addr::to_ipv4_mapped");
    // 1.64 稳定：::ffff:x.y.z.w（v6-mapped v4）转出 Some(Ipv4Addr)。
    // 1.63 期加了 to_ipv4_mapped 原语；1.64 正式列出。
    use std::net::{Ipv4Addr, Ipv6Addr};
    let mapped = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc0a8, 0x0001); // ::ffff:192.168.0.1
    println!("  to_ipv4_mapped() = {:?}", mapped.to_ipv4_mapped());
    println!("  普通 v6 to_ipv4_mapped = {:?}", Ipv6Addr::LOCALHOST.to_ipv4_mapped());
    println!("  已可用：Ipv4Addr::to_ipv6_mapped -> {}", Ipv4Addr::new(1, 2, 3, 4).to_ipv6_mapped());

    // ============================================================
    println!("\n5. write! 进 OsString（fmt::Write）");
    use std::ffi::OsString;
    use std::fmt::Write as _;
    let mut os = OsString::from("n=");
    write!(os, "{} + {}", 1, 2).unwrap();
    println!("  write!(OsString) 结果 = {:?}", os);

    // ============================================================
    println!("\n6. core/alloc::ffi 类型路径扩展");
    // CStr/CString 从 std::ffi 变为 core/alloc::ffi 可用（no-std 场景）：
    println!("  core::ffi::CStr / alloc::ffi::CString 已还可从 std::ffi 用。");
    let c = std::ffi::CString::new("open the way").unwrap();
    println!("  CString 照常工作 = {:?}", c);

    // ============================================================
    println!("\n7. 兼容性提醒");
    println!("  - linux-gnu 最低 kernel 3.2 / glibc 2.17。");
    println!("  - 网络基础类型改 Rust 理想布局，transmute 这些类型会崩。");
    println!("  - BTreeMap soundness 修复；C-like 枚举 -> int cast Drop 行为变了。");
}
