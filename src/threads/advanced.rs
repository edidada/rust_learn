// 高级线程 API 示例
use std::thread;
use std::panic;

fn main() {
    // 5.1 作用域线程 (不会泄漏，自动join)
    println!("5.1 作用域线程示例:");
    let data = vec![1, 2, 3];
    
    thread::scope(|scope| {
        for i in 0..3 {
            scope.spawn(|| {
                println!("作用域线程 {} 访问数据: {:?}", i, data);
            });
        }
        // 这里会自动等待所有作用域线程结束
    });
    
    println!("所有作用域线程已完成");
    
    // 5.2 无恐慌线程
    println!("\n5.2 无恐慌线程示例:");
    if let Ok(handle) = panic::catch_unwind(|| {
        thread::spawn(|| {
            // 这个线程不会让整个程序崩溃
            panic!("但会被捕获");
        })
    }) {
        // 处理结果
        match handle.join() {
            Ok(_) => println!("线程正常结束"),
            Err(e) => println!("线程恐慌被捕获: {:?}", e),
        }
    }
    
    println!("程序继续执行，未被线程恐慌影响");
}