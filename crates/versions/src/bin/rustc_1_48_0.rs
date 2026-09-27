// rustc 1.48.0 演示 —— unsafe mod 解析 / 数组 TryFrom<Vec> / matches! 尾逗号 / &Stdout Write /
// make_contiguous / future::ready·pending / as_ptr_range / const 判定方法 / intra-doc link
use std::cmp::Ordering;
use std::future::{pending, ready};
use std::io::Write;
use std::pin::pin;
use std::task::Context;

fn main() {
    println!("rustc 1.48.0 演示");

    // 1.48.0：任意长度数组实现 TryFrom<Vec<T>>（此前仅长度 ≤ 32）。
    println!("\n1. 数组 TryFrom<Vec<T>>（任意长度）");
    let v33 = (1..=33).collect::<Vec<u8>>();
    // try_into 失败（长度不匹配）返回 Err，不会 panic：
    let arr: [u8; 33] = <[u8; 33]>::try_from(v33.clone()).expect("长度恰为 33");
    println!("33 元素 Vec 转成 [u8; 33] 成功：len = {}，末元素 = {}", arr.len(), arr[32]);
    let _too_long: Result<[u8; 32], _> = <[u8; 32]>::try_from(v33);
    println!("33 个元素转 [u8; 32] = Err（长度不符，可安全处理）");

    // 1.48.0：matches! 支持尾逗号；Vec<A> 实现 PartialEq<[B]>。
    println!("\n2. matches! 尾逗号 / Vec<A> PartialEq<[B]>");
    let ok = matches!(2u8, 1 | 2 | 3,); // 模式后带尾逗号，1.48 起合法
    println!("matches!(2, 1 | 2 | 3,) = {}", ok);
    let vec = vec![1i32, 2, 3];
    let slice: &[i32] = &[1, 2, 3];
    println!("Vec 与切片逐元素相等：{}", vec == slice);

    // 1.48.0：&ChildStdin、&Sink、&Stdout、&Stderr 实现 io::Write。
    println!("\n3. &Stdout / &Sink 实现 io::Write");
    let mut out = &std::io::stdout(); // 借用也能写：write! 作用在 &Stdout 上
    write!(out, "(经 &Stdout 写出，无换行) ").ok();
    out.flush().ok();
    let mut sink = std::io::sink(); // Sink：写入的数据被直接丢弃
    writeln!(sink, "写进 Sink 的这行会被吞掉").ok();
    println!("（flush 后换行，Sink 写入已丢弃）");

    // 1.48.0 稳定：VecDeque::make_contiguous —— 旋转环形缓冲，使元素连续。
    println!("\n4. VecDeque::make_contiguous");
    let mut dq = std::collections::VecDeque::from([1, 2, 3, 4, 5]);
    dq.rotate_left(2); // 破坏连续性：现在是 [3,4,5,1,2] 的环形排布
    let contiguous = dq.make_contiguous(); // 旋转成一段连续内存，返回 &mut [T]
    println!("旋转后的队列连续化 = {:?}", contiguous);

    // 1.48.0 稳定：future::ready / future::pending。手写 poll 演示，无需异步运行时。
    println!("\n5. future::ready / future::pending");
    let waker = std::task::Waker::noop(); // 空操作 waker（1.85+ 稳定，演示用足够）
    let mut cx = Context::from_waker(&waker);
    let mut r = pin!(ready(7u8));
    let mut p = pin!(pending::<u8>());
    let rr = r.as_mut().poll(&mut cx);
    let pp = p.as_mut().poll(&mut cx);
    println!("ready(7).poll()      = {:?}", rr);
    println!("pending::<u8>().poll() = {:?}（永远 Pending，永不完成）", pp);

    // 1.48.0 稳定：slice::as_ptr_range / as_mut_ptr_range。
    println!("\n6. slice::as_ptr_range / as_mut_ptr_range");
    let data = [10i32, 20, 30];
    let range = data.as_ptr_range(); // 首末指针构成的 Range<*const i32>
    println!("ptr_range.start == data.as_ptr() = {}", range.start == data.as_ptr());
    let end_off = range.end as usize - range.start as usize;
    println!("区间跨度 = {} 字节（3 × i32）", end_off);

    // 1.48.0：Option/Result 判定与 as_ref、Ordering::reverse/then 成为 const fn。
    println!("\n7. const：Option/Result 判定与 as_ref、Ordering::reverse/then");
    const HAS: bool = Some(1).is_some();
    const NONE: bool = Option::<i32>::None.is_none();
    const RES_OK: bool = Ok::<i32, ()>(2).is_ok();
    const OPT_REF: Option<&i32> = Some(5).as_ref(); // const 里拿借用
    const REV: Ordering = Ordering::Less.reverse();
    const THEN: Ordering = Ordering::Equal.then(Ordering::Greater);
    println!("const Some(1).is_some() = {}", HAS);
    println!("const None.is_none()    = {}", NONE);
    println!("const Ok(2).is_ok()     = {}", RES_OK);
    println!("const Some(5).as_ref()  = {:?}", OPT_REF);
    println!("const Less.reverse()    = {:?}", REV);
    println!("const Equal.then(Greater) = {:?}", THEN);

    // 1.48.0 语言/Rustdoc：unsafe mod 可被语法解析（语义仍拒绝）；
    // intra-doc link 与 #[doc(alias)] 生效于文档注释，运行期无影响，此处仅说明。
    println!("\n8. unsafe mod 解析 / rustdoc intra-doc link");
    // `unsafe mod foo {}` 现在可被过程宏解析为 AST，但语义检查仍报错，故不实际写出。
    // 文档注释里写 `Uses [`std::vec::Vec`]` 会生成指向 std::vec::Vec 的链接（intra-doc link）。
    // `#[doc(alias = "向量")]` 可给项加搜索别名。两者均为文档期特性，见 1.48.0.md。
    let _demo: std::vec::Vec<u8> = std::vec::Vec::new(); // intra-doc 目标示例：std::vec::Vec
    println!("详见 notes/1.48.0.md 的 Language 与 Rustdoc 节");
}
