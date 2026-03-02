// loop 关键字示例

fn main() {
    // 1. 基本无限循环
    println!("1. Basic infinite loop:");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter >= 5 {
            break;
        }
    }
    
    // 2. 带标签的循环
    println!("\n2. Labeled loop:");
    let mut outer_counter = 0;
    'outer: loop {
        outer_counter += 1;
        println!("Outer counter: {}", outer_counter);
        
        let mut inner_counter = 0;
        loop {
            inner_counter += 1;
            println!("Inner counter: {}", inner_counter);
            if inner_counter >= 2 {
                break 'outer; // 跳出外部循环
            }
        }
    }
    
    // 3. 从循环中返回值
    println!("\n3. Returning value from loop:");
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("Result from loop: {}", result);
}