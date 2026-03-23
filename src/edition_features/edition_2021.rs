//! Rust 2021 Edition 特性说明
//!
//! Rust 2021 Edition于2021年10月发布，带来了闭包捕获、panic宏等方面的改进。

fn main() {
    println!("=== Rust 2021 Edition 新增特性 ===\n");

    // 1. 闭包捕获规则改进
    println!("1. 闭包捕获规则改进 (Closure Capture)");
    closure_capture_demo();

    // 2. panic! 宏一致性
    println!("\n2. panic! 宏一致性");
    panic_macro_demo();

    // 3. 数组IntoIterator
    println!("\n3. 数组实现IntoIterator");
    array_into_iterator_demo();

    // 4. 保留语法
    println!("\n4. 保留语法 (Reserved Syntax)");
    reserved_syntax_demo();

    // 5. 字符串格式化改进
    println!("\n5. 格式化字符串改进");
    format_string_demo();

    // 6. 新预导入模块
    println!("\n6. 预导入模块 (Prelude) 更新");
    prelude_demo();
}

// 1. 闭包捕获规则改进
fn closure_capture_demo() {
    // Rust 2021改进了闭包如何捕获变量
    // 现在闭包只捕获实际使用的字段，而不是整个结构体

    struct Config {
        value: String,
        count: i32,
    }

    let config = Config {
        value: String::from("hello"),
        count: 5,
    };

    // 在Rust 2021中，这个闭包只捕获config.count
    // 而不是整个config
    let closure = || {
        println!("   只使用count: {}", config.count);
    };

    // 因此config.value仍然可以使用
    println!("   config.value仍然可用: {}", config.value);
    closure();

    println!("   Rust 2021: 闭包只捕获实际使用的字段");
}

// 2. panic! 宏一致性
fn panic_macro_demo() {
    // Rust 2021统一了panic!宏的行为

    // 在Rust 2021中，panic!只接受字符串字面量
    // panic!("简单的panic消息");

    // 如果需要格式化，必须使用format!
    let value = 42;
    // panic!("值是: {}", value); // 在Rust 2021中会警告
    // 应该使用:
    // panic!("值是: {value}", value = value);

    println!("   panic!宏行为更加一致");
    println!("   推荐使用panic!(\"消息{{value}}\", value = value)形式");
}

// 3. 数组实现IntoIterator
fn array_into_iterator_demo() {
    // Rust 2021中，数组直接实现IntoIterator
    let arr = [1, 2, 3, 4, 5];

    // 可以直接在for循环中使用数组
    print!("   数组迭代: ");
    for item in arr {
        print!("{} ", item);
    }
    println!();

    // 也可以使用.into_iter()
    let sum: i32 = arr.into_iter().sum();
    println!("   数组求和: {}", sum);

    // 注意：这会消耗数组
    // 如果不想消耗，使用.iter()
    let arr2 = [10, 20, 30];
    print!("   使用iter(): ");
    for item in arr2.iter() {
        print!("{} ", item);
    }
    println!();
}

// 4. 保留语法
fn reserved_syntax_demo() {
    // Rust 2021保留了某些语法用于未来扩展
    // 例如：|...| 闭包语法

    // 以下语法在Rust 2021中被保留，但尚未实现：
    // let _closure = |...args| args.len();

    println!("   保留了|...|语法用于未来扩展");
    println!("   为语言的未来发展预留空间");
}

// 5. 格式化字符串改进
fn format_string_demo() {
    // Rust 2021支持在格式化字符串中直接捕获变量
    let name = "Rust";
    let version = 2021;

    // 可以直接在字符串中使用变量名
    let msg = format!("欢迎使用{name} {version}!");
    println!("   {}", msg);

    // 也支持表达式
    let x = 5;
    let y = 10;
    println!("   {x} + {y} = {}", x + y);

    println!("   格式化字符串可以直接捕获变量");
}

// 6. 预导入模块更新
fn prelude_demo() {
    // Rust 2021的std::prelude增加了一些常用类型

    // TryFrom和TryTrait现在更容易使用
    use std::convert::TryFrom;

    let num = i32::try_from(100i64);
    match num {
        Ok(n) => println!("   转换成功: {}", n),
        Err(_) => println!("   转换失败"),
    }

    // Option和Result的更多方法可用
    let opt: Option<i32> = Some(42);
    if let Some(val) = opt {
        println!("   Option值: {}", val);
    }

    println!("   预导入模块包含更多常用trait和类型");
}

// 其他Rust 2021改进
#[allow(dead_code)]
fn other_improvements() {
    // 1. 字面量格式化改进
    // 更好地支持二进制、八进制、十六进制字面量

    // 2. Cargo改进
    // - 默认使用2021 edition
    // - 改进了依赖解析

    // 3. 编译器诊断改进
    // 更好的错误信息和提示

    // 4. 模式匹配穷尽性检查改进
    // 更好的警告和错误提示
}
