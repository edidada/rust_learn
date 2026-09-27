// rustc 1.85.0 演示 —— Edition 2024（r#gen、unsafe 属性）、Waker::noop、midpoint、tuple Extend
// 注意：本仓库本身就是 edition 2024，1.85 稳定的一切在 1.98 可用。
use std::sync::Arc;

// Edition 2024：unsafe 属性必须写成 #[unsafe(...)]。
// 旧写法 #[no_mangle] 在 2024 edition 是硬错误；本文件已在 edition 2024 下演示。
// no_mangle 的作用只是让符号按原名导出，在普通二进制里也能编译。
#[unsafe(no_mangle)]
extern "C" fn rustlearn_2024_export() -> u32 {
    2024
}

// 1.85 稳定 #[diagnostic::do_not_recommend]：抑制该 impl 作为"建议"出现在诊断里。
// 本地 trait + 本地类型，演示属性本身（真实用途是给 blanket impl 减噪）：
trait LocalFrom<E> {
    fn local_from(e: E) -> Self;
}
#[diagnostic::do_not_recommend]
impl<T: Clone> LocalFrom<Nothing> for Vec<T> {
    fn local_from(_: Nothing) -> Self {
        Vec::new()
    }
}
struct Nothing;

fn main() {
    println!("rustc 1.85.0 演示");

    println!("\n1. Edition 2024：`gen` 成为保留关键字 → 用 `r#gen`");
    // 旧 edition 里 gen 是普通标识符；2024 起它是保留字（为 async 迭代器 gen 块铺路）。
    let r#gen = 42; // ← 必须用 raw identifier
    println!("r#gen = {gen}（gen 单独写会报错，这里 r#gen 解引用后可简称 gen 于格式串）");
    // 反例注释：let gen = 42;  ← edition 2024 下 error: `gen` 是保留关键字

    println!("\n2. Edition 2024：unsafe 属性 #[unsafe(no_mangle)]");
    println!("rustlearn_2024_export() = {}", rustlearn_2024_export());
    // 属性宏语法 #[unsafe(...)] 告诉编译器"这是不安全承诺"，让安全审计一眼可辨。

    println!("\n3. Waker::noop + 手写最小执行器（为无 tokio 场景讲解 async）");
    // 1.85 稳定 Waker::noop()：一个什么都不做的 Waker，适合"确定不会 Pending"的场景。
    // 用它轮询一个立即完成的 future（std-only，无需任何运行时）：
    let fut = std::future::ready(7u32);
    let mut fut = Box::pin(fut);
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    match fut.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(v) => println!("轮询 ready future → {v}（Waker::noop，1.85 稳定）"),
        std::task::Poll::Pending => println!("Pending（noop waker 永远等不到）"),
    }
    // async closures（1.85 稳定，RFC 3668）：async || { … }
    // 与普通 async fn 返回值不同：闭包能捕获借用。这里用同样的手写执行器跑一次。
    let base = 10;
    let ac = async move || base * 3; // async closure
    let mut ac_fut = Box::pin(ac());
    if let std::task::Poll::Ready(v) = ac_fut.as_mut().poll(&mut cx) {
        println!("async closure 捕获 base=10 → {v}（AsyncFnOnce 输出）");
    }

    println!("\n4. midpoint（1.85 稳定）");
    println!("7u32.midpoint(10) = {}", 7u32.midpoint(10)); // 无溢出的中点
    println!("1.0f64.midpoint(2.0) = {}", 1.0f64.midpoint(2.0));

    println!("\n5. tuple 的 Extend / FromIterator（arity 1..=12，1.85 稳定）");
    let (a, b): (Vec<i32>, Vec<char>) = [(1, 'x'), (2, 'y')].into_iter().collect();
    println!("collect() 到 tuple = ({a:?}, {b:?})");
    let mut pair = (Vec::<i32>::new(), Vec::<char>::new());
    std::iter::zip([1, 2], ['a', 'b']).for_each(|(x, _y)| pair.0.push(x));
    let _ = pair;
    println!("pair = {pair:?}（tuple Extend 机制演示）");

    println!("\n6. Arc 共享与 LocalFrom（do_not_recommend 实际调用）");
    let a1: Arc<i32> = Arc::new(5);
    let a2 = Arc::clone(&a1);
    println!("Arc 引用计数 = {}", Arc::strong_count(&a1));
    println!("两个 Arc 指向同一内存 = {}", Arc::ptr_eq(&a1, &a2));
    let _from_nothing: Vec<i32> = LocalFrom::local_from(Nothing);
    println!("LocalFrom<Nothing> 调用成功（#[diagnostic::do_not_recommend] 已标注 impl）");
}
