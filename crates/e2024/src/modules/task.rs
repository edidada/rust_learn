// task 模块示例：用于处理异步任务的类型和特性
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// 1. 自定义Future
struct SimpleFuture {
    completed: bool,
}

impl Future for SimpleFuture {
    type Output = i32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready(42)
        } else {
            self.completed = true;
            // 通知waker任务已完成
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 2. 使用Waker
struct WakeableTask {
    waker: Option<Waker>,
    done: Arc<AtomicBool>,
}

impl WakeableTask {
    fn new() -> Self {
        Self {
            waker: None,
            done: Arc::new(AtomicBool::new(false)),
        }
    }
    
    fn set_waker(&mut self, waker: Waker) {
        self.waker = Some(waker);
    }
    
    fn mark_done(&self) {
        self.done.store(true, Ordering::Relaxed);
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }
}

impl Future for WakeableTask {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = self.get_mut();
        
        if self_mut.done.load(Ordering::Relaxed) {
            Poll::Ready(())
        } else {
            self_mut.set_waker(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    // 1. 使用自定义Future
    println!("1. Using custom Future:");
    let future = SimpleFuture { completed: false };
    let result = future.await;
    println!("Future result: {}", result);
    
    // 2. 使用Waker
    println!("\n2. Using Waker:");
    let mut task = WakeableTask::new();
    let task_clone = task.done.clone();
    
    // 在另一个线程中标记任务完成
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        task_clone.store(true, Ordering::Relaxed);
        println!("Task marked as done");
    });
    
    // 等待任务完成
    task.await;
    println!("Task completed");
    
    println!("Task module examples completed");
}