//! Rust 2015 Edition 特性说明
//!
//! Rust 2015是Rust的第一个稳定版本（1.0），于2015年5月发布。

// 2015 风格：依赖库需要 extern crate 声明，并可用 as 重命名（2015 重命名第三方依赖的常见做法）。
// std 已由编译器默认注入，直接 `extern crate std;` 会重复定义，故用 as 绑定新名字。
// 2018 起 extern crate 不再必需，直接 use 路径即可。
extern crate std as rust_std;

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

    // 8. 模块系统（2015 风格）
    println!("\n8. 模块系统 (Modules, 2015 风格)");
    module_system_demo();

    // 9. macro_rules! 声明宏
    println!("\n9. macro_rules! 声明宏 (Declarative Macros)");
    macro_rules_demo();
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

// 8. 模块系统演示（2015 风格）
fn module_system_demo() {
    // 2015：use 路径以 crate 根为绝对路径，可显式以 :: 开头
    // 通过顶部 `extern crate std as rust_std;` 重命名后的名字访问
    use ::rust_std::collections::HashMap;

    let mut ages: HashMap<&str, u32> = HashMap::new();
    ages.insert("Rust 2015", 2015);
    println!("   extern crate 重命名 + 绝对路径导入: {:?}", ages);

    // 2015：第三方依赖需 extern crate 声明（见文件顶部）
    // 2018 起 extern crate 不再必需
    println!("   extern crate 是 2015 的标志性语法，2018 起可省略");
}

// 9. macro_rules! 声明宏演示
// 2015 中导入外部 crate 的宏用 #[macro_use] extern crate，本宏定义在同一文件内直接可用
macro_rules! describe {
    ($value:expr) => {
        println!("   macro_rules! 宏展开: {} 是 {} 的演示", $value, "Rust 2015")
    };
}

fn macro_rules_demo() {
    describe!("所有权、借用与生命周期");

    let sum = 1 + 2;
    let computed = {
        macro_rules! add {
            ($a:expr, $b:expr) => {
                $a + $b
            };
        }
        add!(sum, 10)
    };
    println!("   宏作为表达式: 1 + 2 + 10 = {}", computed);
}
