// await 关键字示例

use std::time::Duration;
use tokio::time::sleep;

async fn slow_operation(ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("Operation completed after {}ms", ms)
}

async fn parallel_operations() {
    // 启动多个异步操作
    let op1 = slow_operation(1000);
    let op2 = slow_operation(500);
    let op3 = slow_operation(800);
    
    // 等待所有操作完成
    let (result1, result2, result3) = tokio::join!(
        op1,
        op2,
        op3
    );
    
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    println!("Result 3: {}", result3);
}

#[tokio::main]
async fn main() {
    println!("Starting operations");
    
    // 等待单个操作
    let result = slow_operation(1000).await;
    println!("Single operation result: {}", result);
    
    // 并行操作
    parallel_operations().await;
    
    println!("All operations completed");
}