// 线程本地存储 (TLS) 示例 - 对应 pthread_key
use std::cell::RefCell;
use std::thread;

fn main() {
    // 2.1 thread_local! 宏
    println!("2.1 thread_local! 宏示例:");
    thread_local! {
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }
    
    // 在不同线程中访问
    let handle1 = thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() = 1;
            println!("线程1计数器: {}", c.borrow());
        });
    });
    
    let handle2 = thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() = 2;
            println!("线程2计数器: {}", c.borrow());
        });
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    // 2.2 通过 with 访问
    println!("\n2.2 通过 with 访问线程本地存储:");
    thread_local! {
        static THREAD_ID: u64 = thread::current().id().as_u64();
    }
    
    THREAD_ID.with(|id| {
        println!("当前线程ID: {:?}", id);
    });
    
    // 验证不同线程的本地存储是独立的
    let handle3 = thread::spawn(|| {
        THREAD_ID.with(|id| {
            println!("新线程ID: {:?}", id);
        });
    });
    
    handle3.join().unwrap();
}