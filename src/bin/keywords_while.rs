// while 关键字示例

fn main() {
    // 1. 基本while循环
    println!("1. Basic while loop:");
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
    
    // 2. 使用while循环处理输入
    println!("\n2. While loop with input:");
    let mut number = 1;
    while number <= 10 {
        if number % 2 == 0 {
            println!("Even number: {}", number);
        }
        number += 1;
    }
    
    // 3. 无限循环与break
    println!("\n3. Infinite loop with break:");
    let mut countdown = 5;
    while true {
        println!("Countdown: {}", countdown);
        countdown -= 1;
        if countdown < 0 {
            break;
        }
    }
    
    // 4. while let 模式匹配
    println!("\n4. While let pattern matching:");
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 3 {
            optional = None;
        } else {
            println!("Current value: {}", i);
            optional = Some(i + 1);
        }
    }
}