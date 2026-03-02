// borrow 模块示例：借用数据操作
use std::borrow::{Borrow, Cow};

fn main() {
    // Borrow trait示例
    let s = String::from("Hello");
    let borrowed: &str = s.borrow();
    println!("Borrowed string: {}", borrowed);
    
    // Cow（Copy on Write）示例
    // 不可变借用
    let cow_str: Cow<str> = Cow::Borrowed("Hello");
    println!("Cow::Borrowed: {}", cow_str);
    
    // 所有权
    let owned_str = String::from("World");
    let cow_owned: Cow<str> = Cow::Owned(owned_str);
    println!("Cow::Owned: {}", cow_owned);
    
    // 修改Cow（会自动转换为Owned）
    let mut cow = Cow::Borrowed("Hello");
    cow.to_mut().push_str(" World");
    println!("Modified Cow: {}", cow);
    println!("Is Cow owned now? {}", matches!(cow, Cow::Owned(_)));
}