// sync 模块示例：有用的同步原语
use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Mutex
    println!("1. Mutex:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Mutex counter: {:?}", *counter.lock().unwrap());
    
    // 2. RwLock
    println!("\n2. RwLock:");
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // 读取线程
    for i in 0..3 {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let read_data = data.read().unwrap();
            println!("Reader {}: {:?}", i, *read_data);
            thread::sleep(Duration::from_millis(100));
        });
    }
    
    // 写入线程
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut write_data = data.write().unwrap();
        write_data.push(4);
        println!("Writer: Added 4, data: {:?}", *write_data);
    });
    
    // 3. Barrier
    println!("\n3. Barrier:");
    let barrier = Arc::new(Barrier::new(3));
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("Thread {}: Before barrier", i);
            barrier.wait();
            println!("Thread {}: After barrier", i);
        });
    }
    
    // 4. Condvar
    println!("\n4. Condvar:");
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    // 等待线程
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("Worker thread: Started");
    });
    
    // 通知线程
    thread::sleep(Duration::from_millis(100));
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    *started = true;
    println!("Main thread: Notifying worker");
    cvar.notify_one();
    
    // 等待所有线程完成
    thread::sleep(Duration::from_millis(500));
    println!("All examples completed");
}