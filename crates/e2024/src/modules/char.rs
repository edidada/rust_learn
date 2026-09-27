// char 模块示例：字符操作
use std::char;

fn main() {
    // 字符创建
    let c = 'A';
    println!("Character: {}", c);
    
    // 字符属性
    println!("Is alphabetic? {}", c.is_alphabetic());
    println!("Is numeric? {}", c.is_numeric());
    println!("Is whitespace? {}", c.is_whitespace());
    println!("Is lowercase? {}", c.is_lowercase());
    println!("Is uppercase? {}", c.is_uppercase());
    
    // 字符转换
    println!("To lowercase: {}", c.to_lowercase());
    println!("To uppercase: {}", c.to_uppercase());
    
    // Unicode码点
    println!("Unicode code point: {:x}", c as u32);
    
    // 从Unicode码点创建字符
    let heart = char::from_u32(0x2665).unwrap();
    println!("Heart symbol: {}", heart);
    
    // 字符迭代
    let s = "Hello 世界"; 
    println!("Characters in '{}':", s);
    for ch in s.chars() {
        println!("{}", ch);
    }
}