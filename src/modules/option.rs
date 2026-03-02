// option 模块示例：可选值
use std::option::Option;

fn main() {
    // 1. 创建Option值
    println!("1. Creating Option values:");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    println!("some_value: {:?}", some_value);
    println!("none_value: {:?}", none_value);
    
    // 2. 匹配Option
    println!("\n2. Matching Option:");
    match some_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("Got nothing"),
    }
    
    match none_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("Got nothing"),
    }
    
    // 3. 使用if let
    println!("\n3. Using if let:");
    if let Some(value) = some_value {
        println!("if let: Got value: {}", value);
    }
    
    // 4. 链式方法
    println!("\n4. Chaining methods:");
    let result = some_value
        .map(|x| x * 2)
        .filter(|x| x > &50)
        .unwrap_or(0);
    println!("Chained result: {}", result);
    
    // 5. 提取值
    println!("\n5. Extracting values:");
    println!("unwrap: {}", some_value.unwrap());
    println!("unwrap_or: {}", none_value.unwrap_or(100));
    println!("unwrap_or_else: {}", none_value.unwrap_or_else(|| 200));
    
    // 6. 转换为Result
    println!("\n6. Converting to Result:");
    let result = some_value.ok_or("No value");
    println!("ok_or: {:?}", result);
    
    // 7. 组合Option
    println!("\n7. Combining Option:");
    let a = Some(10);
    let b = Some(20);
    let c = None;
    
    let combined = a.and(b);
    println!("a.and(b): {:?}", combined);
    
    let combined2 = a.and(c);
    println!("a.and(c): {:?}", combined2);
    
    let combined3 = a.or(c);
    println!("a.or(c): {:?}", combined3);
}