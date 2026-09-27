// rustc 1.36.0 演示 —— Future/任务组件入库 / MaybeUninit / copied / rotate / dbg! 多参
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn main() {
    println!("rustc 1.36.0 演示");
    // 1.36.0 把 Future 从外部库（futures 0.1 等）收进入标准库：
    // std::future::Future、std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable} 全部稳定。
    // 这为 1.39.0 的 async/await 稳定铺路。这里手写一个最简单的 Future 并手动轮询。
    println!("\n1. future::Future / task::Poll 入库");
    let mut counter = CounterFuture { polled: 0 };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    // CounterFuture 不实现 Unpin，用 Pin<&mut _> 轮询（futures 0.1 时代是 poll_mut）
    let mut pinned = Pin::new(&mut counter);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(n) => {
                println!("Future 完成，共被轮询 {} 次，返回 {}", counter.polled, n);
                break;
            }
            Poll::Pending => {
                println!("Poll::Pending，第 {} 次，继续轮询（真实运行时这里会挂起等待 waker）", counter.polled);
            }
        }
    }

    println!("\n2. mem::MaybeUninit（1.36 稳定的安全初始化抽象）");
    // 1.36 引入 MaybeUninit 作为 mem::uninitialized 的替代品；
    // 对 [u8; 16] 这类允许"未初始化字节"的类型可以安全地用 zeroed 直接构造。
    let buf: [u8; 16] = unsafe { std::mem::zeroed() };
    println!("MaybeUninit/zeroed 得到全零缓冲: {:?}", buf);

    println!("\n3. Iterator::copied");
    let refs: Vec<&u8> = vec![&1, &2, &3];
    let owned: Vec<u8> = refs.iter().copied().collect(); // 1.36 稳定，等价 .cloned()，但语义限定 Copy
    println!("&[&1,&2,&3].iter().copied() = {:?}", owned);

    println!("\n4. VecDeque::rotate_left / rotate_right");
    use std::collections::VecDeque;
    let mut dq: VecDeque<i32> = (1..=5).collect();
    dq.rotate_left(2); // 1.36 稳定：整体左旋 2 位
    println!("rotate_left(2) = {:?}", dq);
    dq.rotate_right(1); // 1.36 稳定：再右旋 1 位
    println!("rotate_right(1) = {:?}", dq);

    println!("\n5. dbg! 多参数（1.36 起）");
    // dbg!(a, b) 会逐个打印表达式并返回元组 (a, b)
    let (a, b) = dbg!(10, 20);
    println!("dbg!(10, 20) 返回元组：a = {}, b = {}", a, b);
}

/// 一个演示用 Future：前两次轮询返回 Pending，第三次返回 Ready(polled)
struct CounterFuture {
    polled: u32,
}

impl Future for CounterFuture {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u32> {
        self.polled += 1;
        if self.polled >= 3 {
            Poll::Ready(self.polled)
        } else {
            Poll::Pending
        }
    }
}

/// 构造一个"什么都不做"的 Waker（用 RawWaker 闭包表）
fn noop_waker() -> Waker {
    // 裸指针与 vtable 需要配合 RawWakerVTable 使用；这里演示 1.36 就位的最底层 waker API。
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}
