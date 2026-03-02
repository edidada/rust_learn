// ascii 模块示例：ASCII字符串和字符操作
use std::ascii::AsciiExt;

fn main() {
    // 字符操作
    let c = 'A';
    println!("Is '{}' ASCII? {}", c, c.is_ascii());
    println!("Is '{}' uppercase? {}", c, c.is_ascii_uppercase());
    println!("Is '{}' lowercase? {}", c, c.is_ascii_lowercase());
    println!("To lowercase: {}", c.to_ascii_lowercase());
    println!("To uppercase: {}", c.to_ascii_uppercase());
    
    // 字符串操作
    let s = "Hello, Rust!";
    println!("Original string: {}", s);
    println!("To lowercase: {}", s.to_ascii_lowercase());
    println!("To uppercase: {}", s.to_ascii_uppercase());
    
    // 检查字符串是否全为ASCII
    let ascii_str = "Hello";
    let non_ascii_str = "Hello 世界";
    println!("Is '{}' all ASCII? {}", ascii_str, ascii_str.is_ascii());
    println!("Is '{}' all ASCII? {}", non_ascii_str, non_ascii_str.is_ascii());
}