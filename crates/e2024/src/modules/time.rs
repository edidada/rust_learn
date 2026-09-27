// time 模块示例：时域量化
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // 1. 持续时间
    println!("1. Duration:");
    let duration = Duration::new(1, 500_000_000); // 1.5秒
    println!("Duration: {:?}", duration);
    println!("Seconds: {}", duration.as_secs());
    println!("Nanos: {}", duration.subsec_nanos());
    println!("Total nanos: {}", duration.as_nanos());
    
    // 2. 时间点
    println!("\n2. Instant:");
    let start = Instant::now();
    
    // 模拟一些工作
    std::thread::sleep(Duration::from_millis(500));
    
    let end = Instant::now();
    let elapsed = end.duration_since(start);
    println!("Elapsed time: {:?}", elapsed);
    
    // 3. 系统时间
    println!("\n3. System time:");
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);
    
    // 4. Unix时间戳
    println!("\n4. Unix timestamp:");
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!("Seconds since epoch: {}", duration.as_secs());
            println!("Nanos since epoch: {}", duration.subsec_nanos());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // 5. 持续时间操作
    println!("\n5. Duration operations:");
    let a = Duration::from_secs(1);
    let b = Duration::from_millis(500);
    
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("a + b: {:?}", a + b);
    println!("a - b: {:?}", a - b);
    println!("a * 2: {:?}", a * 2);
    println!("b / 2: {:?}", b / 2);
    
    // 6. 时间比较
    println!("\n6. Time comparison:");
    let instant1 = Instant::now();
    std::thread::sleep(Duration::from_millis(100));
    let instant2 = Instant::now();
    
    println!("instant1 < instant2: {}", instant1 < instant2);
    println!("instant1 > instant2: {}", instant1 > instant2);
    println!("instant1 == instant2: {}", instant1 == instant2);
}