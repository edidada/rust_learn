// str 模块示例：str原始类型的工具

fn main() {
    // 1. 基本字符串操作
    println!("1. Basic string operations:");
    let s = "Hello, Rust!";
    
    println!("Original string: {}", s);
    println!("Length: {}", s.len());
    println!("Is empty: {}", s.is_empty());
    
    // 2. 字符串索引
    println!("\n2. String indexing:");
    println!("First character: {}", s.chars().next().unwrap());
    println!("Last character: {}", s.chars().last().unwrap());
    
    // 3. 字符串切片
    println!("\n3. String slicing:");
    println!("Slice from 0 to 5: {}", &s[0..5]);
    println!("Slice from 7 to end: {}", &s[7..]);
    
    // 4. 字符串方法
    println!("\n4. String methods:");
    
    // contains
    println!("Contains 'Rust': {}", s.contains("Rust"));
    
    // starts_with 和 ends_with
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with '!': {}", s.ends_with("!"));
    
    // find
    if let Some(index) = s.find("Rust") {
        println!("Found 'Rust' at index: {}", index);
    }
    
    // replace
    let replaced = s.replace("Rust", "World");
    println!("After replace: {}", replaced);
    
    // 5. 字符串迭代
    println!("\n5. String iteration:");
    println!("Characters:");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();
    
    println!("Bytes:");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();
    
    // 6. 字符串转换
    println!("\n6. String conversion:");
    let num_str = "42";
    let num: i32 = num_str.parse().unwrap();
    println!("String to i32: {} -> {}", num_str, num);
    
    let float_str = "3.14";
    let float: f64 = float_str.parse().unwrap();
    println!("String to f64: {} -> {}", float_str, float);
}