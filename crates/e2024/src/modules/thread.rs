// thread 模块示例：原生线程
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    // 1. 基本线程创建
    println!("1. Basic thread creation:");
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
        thread::sleep(Duration::from_millis(500));
        println!("Thread finished");
    });
    
    println!("Hello from main thread!");
    handle.join().unwrap();
    println!("Main thread finished waiting for spawned thread");
    
    // 2. 线程间传递数据
    println!("\n2. Passing data to threads:");
    let message = String::from("Hello from main");
    let handle = thread::spawn(move || {
        println!("Received: {}", message);
    });
    
    handle.join().unwrap();
    // 注意：message 已经被移动到线程中，不能在主线程中使用
    // println!("Message: {}", message); // 这行会导致编译错误
    
    // 3. 共享可变数据
    println!("\n3. Shared mutable data:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread incremented counter to: {}", *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    // 4. 线程名称
    println!("\n4. Thread names:");
    let handle = thread::Builder::new()
        .name("worker".to_string())
        .spawn(|| {
            println!("Thread name: {:?}", thread::current().name());
        })
        .unwrap();
    
    handle.join().unwrap();
    
    // 5. 线程休眠
    println!("\n5. Thread sleep:");
    println!("Main thread sleeping for 1 second...");
    thread::sleep(Duration::from_secs(1));
    println!("Main thread awake!");
}