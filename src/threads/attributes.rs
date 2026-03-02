// 线程属性与信息示例
use std::thread;

fn main() {
    // 4.1 获取当前线程
    println!("4.1 获取当前线程:");
    let current_thread = thread::current();
    println!("当前线程: {:?}", current_thread);
    
    // 4.2 线程ID (不可克隆，唯一标识)
    println!("\n4.2 线程ID:");
    let thread_id = thread::current().id();
    println!("线程ID: {:?}", thread_id);
    
    // 4.3 线程名称 (调试用)
    println!("\n4.3 线程名称:");
    let handle = thread::Builder::new()
        .name("my-worker-thread".into())
        .spawn(|| {
            println!("线程名: {:?}", thread::current().name());
        })
        .unwrap();
    
    handle.join().unwrap();
    
    // 4.4 设置栈大小
    println!("\n4.4 设置栈大小:");
    let handle = thread::Builder::new()
        .stack_size(4 * 1024 * 1024)  // 4MB 栈空间
        .spawn(|| {
            println!("使用4MB栈空间的线程");
            // 这里可以执行需要大栈空间的操作
        })
        .unwrap();
    
    handle.join().unwrap();
}