// 基本线程创建和管理示例
use std::thread;
use std::time::Duration;

fn main() {
    // 1.1 创建线程 - 对应 pthread_create
    println!("1.1 创建基本线程:");
    let handle = thread::spawn(|| {
        println!("线程开始执行");
        thread::sleep(Duration::from_secs(1));
        println!("线程执行完成");
        // 可以返回值
        42
    });
    
    // 1.2 等待线程完成 - 对应 pthread_join
    println!("1.2 等待线程完成:");
    match handle.join() {
        Ok(value) => println!("线程返回值: {}", value),
        Err(e) => eprintln!("线程恐慌: {:?}", e),
    }
    
    // 1.3 创建带参数的线程
    println!("\n1.3 创建带参数的线程:");
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("线程获得数据所有权: {:?}", data);
        data.len()  // 返回数据长度
    });
    
    match handle.join() {
        Ok(length) => println!("数据长度: {}", length),
        Err(e) => eprintln!("线程恐慌: {:?}", e),
    }
    
    // 1.4 线程恐慌处理
    println!("\n1.4 线程恐慌处理:");
    let handle = thread::spawn(|| {
        panic!("线程崩溃了！");
    });
    
    if handle.join().is_err() {
        println!("线程恐慌被捕获");
    }
}