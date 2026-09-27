//! Rust 2015 Edition 特性说明
//!
//! Rust 2015是Rust的第一个稳定版本（1.0），于2015年5月发布。

fn main() {
    println!("=== Rust 2015 Edition 特性 ===\n");

    // 1. 所有权系统
    println!("1. 所有权系统 (Ownership)");
    ownership_demo();

    // 2. 借用和引用
    println!("\n2. 借用和引用 (Borrowing)");
    borrowing_demo();

    // 3. 生命周期
    println!("\n3. 生命周期 (Lifetimes)");
    lifetime_demo();

    // 4. 模式匹配
    println!("\n4. 模式匹配 (Pattern Matching)");
    pattern_matching_demo();

    // 5. 特质系统
    println!("\n5. 特质系统 (Traits)");
    trait_demo();

    // 6. 错误处理
    println!("\n6. 错误处理 (Error Handling)");
    error_handling_demo();

    // 7. 宏系统
    println!("\n7. 宏系统 (Macros)");
    macro_demo();
}

// 1. 所有权系统演示
fn ownership_demo() {
    let s1 = String::from("hello");
    let s2 = s1; // s1的所有权移动到s2
    // println!("{}", s1); // 错误！s1不再有效
    println!("   s2 = {}", s2);

    let x = 5;
    let y = x; // 基本类型实现Copy trait，可以复制
    println!("   x = {}, y = {} (Copy类型)", x, y);
}

// 2. 借用和引用演示
fn borrowing_demo() {
    let s = String::from("hello");

    // 不可变借用
    let len = calculate_length(&s);
    println!("   '{}' 长度: {}", s, len);

    // 可变借用
    let mut s = String::from("hello");
    change(&mut s);
    println!("   修改后: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// 3. 生命周期演示
fn lifetime_demo() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("   最长字符串: {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 4. 模式匹配演示
fn pattern_matching_demo() {
    let number = 7;

    match number {
        1 => println!("   One"),
        2 | 3 | 5 | 7 | 11 => println!("   质数"),
        13..=19 => println!("   13到19之间的数"),
        _ => println!("   其他数"),
    }

    // if let 简化匹配
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("   值是3");
    }
}

// 5. 特质系统演示
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.location)
    }
}

fn trait_demo() {
    let article = NewsArticle {
        headline: String::from("Rust 1.0发布"),
        location: String::from("全球"),
    };
    println!("   {}", article.summarize());
}

// 6. 错误处理演示
fn error_handling_demo() {
    // Result类型
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("   成功: {}", value),
        Err(e) => println!("   错误: {}", e),
    }

    // Option类型
    let some_number: Option<i32> = Some(5);
    match some_number {
        Some(n) => println!("   有值: {}", n),
        None => println!("   无值"),
    }
}

// 7. 宏系统演示
fn macro_demo() {
    // vec! 宏
    let v = vec![1, 2, 3];
    println!("   vec! 宏创建的向量: {:?}", v);

    // println! 宏
    println!("   这是println!宏的输出");

    // format! 宏
    let s = format!("Hello, {}!", "Rust 2015");
    println!("   format! 宏: {}", s);
}
