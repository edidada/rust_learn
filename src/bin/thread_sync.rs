// 线程控制与同步示例
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    // 3.1 线程睡眠 - 对应 sleep/usleep
    println!("3.1 线程睡眠示例:");
    println!("开始睡眠...");
    thread::sleep(Duration::from_millis(500));  // 睡眠500毫秒
    println!("睡眠500毫秒后");
    
    thread::sleep(Duration::from_secs_f32(0.5)); // 睡眠0.5秒
    println!("再睡眠0.5秒后");
    
    // 3.2 让出CPU - 对应 sched_yield
    println!("\n3.2 让出CPU示例:");
    println!("让出CPU前");
    thread::yield_now();
    println!("让出CPU后");
    
    // 3.3 屏障同步 - 对应 pthread_barrier
    println!("\n3.3 屏障同步示例:");
    let barrier = Arc::new(Barrier::new(3));  // 等待3个线程
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程 {} 到达屏障", i);
            barrier_clone.wait();  // 等待所有线程
            println!("线程 {} 通过屏障", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("所有线程都通过了屏障");
}