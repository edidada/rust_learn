// static 关键字示例

// 1. 基本静态变量
static mut COUNTER: i32 = 0;
static MESSAGE: &str = "Hello, static!";

// 2. 静态常量
const PI: f64 = 3.141592653589793;

// 3. 静态字符串
static HELLO: &str = "Hello from static";

fn main() {
    println!("Using static keyword example");
    
    // 访问静态字符串
    println!("MESSAGE: {}", MESSAGE);
    println!("HELLO: {}", HELLO);
    
    // 访问静态常量
    println!("PI: {}", PI);
    
    // 修改可变静态变量（需要unsafe）
    unsafe {
        println!("Initial COUNTER: {}", COUNTER);
        COUNTER += 1;
        println!("Updated COUNTER: {}", COUNTER);
    }
    
    // 多次修改
    for i in 0..3 {
        unsafe {
            COUNTER += 1;
            println!("COUNTER after iteration {}: {}", i + 1, COUNTER);
        }
    }
}