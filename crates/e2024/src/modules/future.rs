// future 模块示例：异步基本功能
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

// 1. 自定义Future
struct Delay {
    duration: Duration,
}

impl Future for Delay {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 这里简化实现，实际应该使用定时器
        println!("Polling Delay future");
        // 模拟异步操作
        Poll::Ready(())
    }
}

// 2. 组合Future
async fn async_function() {
    println!("Async function started");
    // 等待1秒
    sleep(Duration::from_secs(1)).await;
    println!("Async function completed");
}

// 3. 使用Future trait
fn run_future<F: Future<Output = T>, T>(future: F) -> T {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(future)
}

#[tokio::main]
async fn main() {
    // 1. 使用自定义Future
    println!("1. Using custom Future:");
    let delay = Delay { duration: Duration::from_secs(1) };
    run_future(delay);
    
    // 2. 使用异步函数
    println!("\n2. Using async function:");
    async_function().await;
    
    // 3. 组合多个Future
    println!("\n3. Composing multiple futures:");
    let future1 = sleep(Duration::from_millis(500));
    let future2 = sleep(Duration::from_millis(300));
    
    tokio::join!(
        future1,
        future2
    );
    
    println!("All futures completed");
}