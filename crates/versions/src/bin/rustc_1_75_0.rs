// rustc 1.75.0 演示 —— RPITIT、async fn in trait（手动 poll）、Option::as_slice、byte_* 指针
fn main() {
    println!("rustc 1.75.0 演示");

    // ============================================================
    println!("\n1. RPITIT：trait 方法返回 impl Trait");
    // 1.75 稳定：trait 方法可以用 impl Trait 作为返回类型，
    // 具体类型由 impl 决定但对外只暴露 trait bound。
    trait Source {
        fn items(&self) -> impl Iterator<Item = i32> + '_; // RPITIT + 借用 self
        fn limit(&self) -> i32;
    }
    struct Range3;
    impl Source for Range3 {
        fn items(&self) -> impl Iterator<Item = i32> + '_ {
            (0..self.limit())
        }
        fn limit(&self) -> i32 {
            3
        }
    }
    // 用 boxed 或直接收集：返回类型是匿名类型，只能通过 trait bound 使用
    fn collect(s: &impl Source) -> Vec<i32> {
        s.items().collect()
    }
    println!("  RPITIT 迭代 -> {:?}", collect(&Range3)); // [0, 1, 2]

    // ============================================================
    println!("\n2. async fn in trait（1.75 稳定，但需自行驱动执行）");
    // trait 里的 async fn 现在合法；它返回匿名 impl Future。
    // 重要限制：std 没有内置 runtime，1.75 不能直接 spawn async trait 对象执行；
    // 下面用 std::task 手动 poll 演示（无任何外部依赖）。
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    trait Loader {
        async fn load(&self) -> u32; // 异步 trait 方法
    }
    struct Db;
    impl Loader for Db {
        async fn load(&self) -> u32 {
            42 // 立即完成，单次 poll 即 Ready
        }
    }
    struct NoopWake;
    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {} // 手动驱动时无需真正唤醒
    }
    let mut fut = pin!(Db.load());
    let waker = Waker::from(Arc::new(NoopWake));
    let mut cx = Context::from_waker(&waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => println!("  async trait 方法手动 poll 完成 -> {}", v),
        Poll::Pending => println!("  Pending（本例不会出现）"),
    }
    println!("  说明：要跨 runtime 执行仍需外部 runtime；async fn in trait 的");
    println!("        spawn/trait 对象支持在 1.75 仍是 unstable（async_trait 相关 RFC）。");

    // ============================================================
    println!("\n3. Option::as_slice / as_mut_slice");
    let some = Some(5).as_slice();
    let none: Option<i32> = None;
    println!("  Some(5).as_slice() = {:?}, None.as_slice() = {:?}", some, none.as_slice());
    let mut om = Some(String::from("改写"));
    if let [s] = om.as_mut_slice() {
        s.push_str("!");
    }
    println!("  as_mut_slice 修改后 = {:?}", om);

    // ============================================================
    println!("\n4. 指针 byte_* 系列 API");
    let arr = [10u8, 20, 30, 40];
    let base: *const u8 = arr.as_ptr();
    let p2 = unsafe { base.byte_add(2) }; // 按字节偏移（unsafe：可能越过边界）
    println!("  base.byte_add(2) 指向 = {}", unsafe { *p2 });
    let back = unsafe { p2.byte_sub(2) };
    println!("  p2.byte_sub(2) 回到首元素 = {}", unsafe { *back });
    let off = unsafe { p2.byte_offset_from(base) };
    println!("  byte_offset_from = {}", off);

    // ============================================================
    println!("\n5. IP 地址的位运算与 to_canonical");
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let a = Ipv4Addr::new(192, 168, 1, 1);
    let b = Ipv4Addr::new(0, 0, 255, 255);
    println!("  192.168.1.1 & 0.0.255.255 = {}", a & b); // 0.0.1.1
    println!("  192.168.1.1 | 0.0.255.255 = {}", a | b);
    println!("  !192.168.1.1 = {}", !a);
    // IPv4 映射的 IPv6 规范化回 IPv4
    let mapped: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
    println!("  to_canonical(IpAddr::V6(映射)) = {}", IpAddr::V6(mapped).to_canonical());

    // ============================================================
    println!("\n6. BufRead for VecDeque<u8>");
    use std::collections::VecDeque;
    use std::io::BufRead;
    let mut dq: VecDeque<u8> = VecDeque::from(b"line1\nline2\n".to_vec());
    let mut line1 = String::new();
    dq.read_line(&mut line1).unwrap();
    println!("  read_line = {:?}", line1.trim_end());

    // ============================================================
    println!("\n7. const 上下文新 API");
    use std::mem::MaybeUninit;
    const Z: [u8; 4] = unsafe { MaybeUninit::<[u8; 4]>::zeroed().assume_init() };
    println!("  const MaybeUninit::zeroed().assume_init() = {:?}", Z);
    enum E { A }
    // const 稳定的是 mem::discriminant 本身（产生 Discriminant 值）；
    // Discriminant 之间比较用的 PartialEq 不是 const，运行期再比
    const DA: std::mem::Discriminant<E> = std::mem::discriminant(&E::A);
    println!("  const mem::discriminant 可用 = {}", DA == std::mem::discriminant(&E::A));

    // ============================================================
    println!("\n8. 文字说明");
    println!("  - None 的零值表示（NPO）成为正式保证，可依赖优化");
    println!("  - Cargo 支持无版本号清单；自动加入 workspace.members");
    println!("  - const 上下文中未对齐成为硬错误；char 与 u32 大小/对齐一致为正式保证");
}
