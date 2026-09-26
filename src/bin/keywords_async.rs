// async 关键字示例

use std::time::Duration;
use tokio::time::sleep;

// 定义异步函数
async fn async_function() -> i32 {
    println!("Async function started");
    // 模拟异步操作
    sleep(Duration::from_millis(1000)).await;
    println!("Async function completed");
    42
}

// 异步块
async fn async_block_example() {
    let result = async {
        println!("Async block started");
        sleep(Duration::from_millis(500)).await;
        println!("Async block completed");
        "Hello from async block"
    }.await;
    
    println!("Async block result: {}", result);
}

#[tokio::main]
async fn main() {
    println!("Main started");
    
    // 调用异步函数
    let result = async_function().await;
    println!("Async function result: {}", result);
    
    // 调用包含异步块的函数
    async_block_example().await;
    
    println!("Main completed");
}