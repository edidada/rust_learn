// backtrace 模块示例：支持捕获OS线程的堆栈回溯
use std::backtrace::{Backtrace, BacktraceStatus};

fn main() {
    // 1. 基本回溯
    println!("1. Basic backtrace:");
    let backtrace = Backtrace::capture();
    println!("Backtrace: {:?}", backtrace);
    
    // 2. 检查回溯状态
    println!("\n2. Backtrace status:");
    match backtrace.status() {
        BacktraceStatus::Captured => println!("Backtrace was captured successfully"),
        BacktraceStatus::Unsupported => println!("Backtrace is unsupported on this platform"),
        BacktraceStatus::Disabled => println!("Backtrace is disabled"),
    }
    
    // 3. 在函数中捕获回溯
    println!("\n3. Backtrace from function:");
    fn deep_function() {
        let backtrace = Backtrace::capture();
        println!("Backtrace from deep_function:\n{:?}", backtrace);
    }
    
    fn intermediate_function() {
        deep_function();
    }
    
    intermediate_function();
    
    // 4. 格式化回溯
    println!("\n4. Formatted backtrace:");
    let backtrace = Backtrace::capture();
    println!("Formatted backtrace:\n{}", backtrace);
}