// 完整的线程示例
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    // 1. 简单的线程创建
    println!("1. 简单的线程创建:");
    let handle1 = thread::spawn(|| {
        println!("线程1: 开始");
        thread::sleep(Duration::from_secs(1));
        println!("线程1: 结束");
        100
    });
    
    // 2. 带共享状态的线程
    println!("\n2. 带共享状态的线程:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("线程 {}: 计数器值 = {}", i, *num);
        });
        handles.push(handle);
    }
    
    // 3. 通道通信
    println!("\n3. 通道通信:");
    let (tx, rx) = mpsc::channel();
    
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
            println!("发送消息: {}", i);
        });
    }
    
    // 等待所有线程
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 获取结果
    println!("\n4. 结果汇总:");
    println!("计数器值: {}", *counter.lock().unwrap());
    
    // 接收通道消息
    println!("收到的消息:");
    for _ in 0..5 {
        println!("  {}", rx.recv().unwrap());
    }
    
    // 等待线程1完成
    if let Ok(value) = handle1.join() {
        println!("线程1返回值: {}", value);
    }
    
    println!("\n所有线程操作完成");
}