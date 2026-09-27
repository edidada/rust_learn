// string 模块示例：UTF-8编码的可增长字符串
use std::string::String;

fn main() {
    // 1. 创建字符串
    println!("1. Creating strings:");
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World".to_string();
    
    println!("s1: '{:?}' (empty: {})", s1, s1.is_empty());
    println!("s2: '{}'", s2);
    println!("s3: '{}'", s3);
    
    // 2. 字符串操作
    println!("\n2. String operations:");
    let mut s = String::from("Hello");
    
    // push
    s.push(' ');
    s.push_str("Rust");
    println!("After push: '{}'", s);
    
    // len
    println!("Length: {}", s.len());
    
    // capacity
    println!("Capacity: {}", s.capacity());
    
    // reserve
    s.reserve(10);
    println!("Capacity after reserve: {}", s.capacity());
    
    // 3. 字符串连接
    println!("\n3. String concatenation:");
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    
    // 使用 + 运算符
    let s6 = s4 + " " + &s5;
    println!("Using + operator: '{}'", s6);
    
    // 使用 format! 宏
    let s7 = format!("{} {}", "Hello", "Rust");
    println!("Using format! macro: '{}'", s7);
    
    // 4. 字符串切片
    println!("\n4. String slicing:");
    let s8 = String::from("Hello, Rust!");
    let slice = &s8[0..5];
    println!("Slice [0..5]: '{}'", slice);
    
    // 5. 字符串迭代
    println!("\n5. String iteration:");
    println!("Characters:");
    for c in s8.chars() {
        print!("{} ", c);
    }
    println!();
    
    // 6. 字符串搜索
    println!("\n6. String searching:");
    if s8.contains("Rust") {
        println!("Contains 'Rust'");
    }
    
    if let Some(index) = s8.find("Rust") {
        println!("Found 'Rust' at index: {}", index);
    }
    
    // 7. 字符串替换
    println!("\n7. String replacement:");
    let replaced = s8.replace("Rust", "World");
    println!("After replace: '{}'", replaced);
}