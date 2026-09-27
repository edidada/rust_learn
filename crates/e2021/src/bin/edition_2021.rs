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

    // 7. 闭包捕获与部分移动
    println!("\n7. 闭包捕获与部分移动 (Precise Closure Capture)");
    closure_partial_move_demo();

    // 8. 宏片段说明符 expr_2021 预留
    println!("\n8. 宏片段预留 expr_2021 (Reserved Macro Fragment)");
    reserved_macro_fragment_demo();

    // 9. FromIterator 加入 prelude
    println!("\n9. FromIterator 加入 prelude (collect 免导入)");
    from_iterator_prelude_demo();
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
    // Rust 2021 让 panic! 与 format! 的行为完全一致：
    // - 单参数时必须是字符串字面量（旧版可传任意表达式作为 panic 载荷）
    // - 格式化新旧语法都有效：旧式 "{}" 位置参数、新式 "{value}" 内联捕获

    // panic!("简单的panic消息"); // 单参：必须是字面量
    let value = 42;
    // panic!("值是: {}", value); // 旧式语法，仍然有效、无警告
    // panic!("值是: {value}");   // 新式内联捕获，2021 起推荐、更简洁

    println!("   panic! 宏与 format! 行为一致，新旧格式化语法均可");
    println!("   推荐使用 panic!(\"消息 {value}\") 形式，更简洁");
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

    // TryFrom/TryInto 自 Rust 2021 起已加入 std prelude，无需手动导入
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

// 7. 闭包捕获与部分移动
// 这是"重构不友好"感受的一个直接来源：
// 2021 之前的 Edition 中，闭包默认捕获整个变量，即使只用到结构体的一个字段，
// 这会导致"部分移动"后的借用错误；2021 改为只捕获实际用到的字段。
// 同一段涉及部分移动和闭包的代码，在 2018 里编译失败，在 2021 里则能通过。
fn closure_partial_move_demo() {
    struct P {
        name: String,
        age: i32,
    }

    let p = P {
        name: String::from("a"),
        age: 1,
    };
    let n = p.name; // 部分移动：p.name 被移走

    // Rust 2021（精确捕获）：闭包只捕获 p.age，编译通过
    let show = || {
        println!("   闭包只捕获 age: {}", p.age);
    };
    // 若在 Rust 2018 中：闭包整体捕获 p，而 p.name 已被移走，
    // 报错 E0382: borrow of partially moved value: `p`
    show();

    println!("   移走的 name 仍可用: {}", n);
    println!("   2018: 闭包整体捕获 → 编译失败；2021: 精确捕获字段 → 编译通过");
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

// 8. 宏片段说明符 expr_2021 预留
// Rust 1.56（随 2021 Edition）为宏的 expr 片段预留了 expr_2021 说明符：
// 未来 expr 片段的行为发生变化时，已使用 expr_2021 的宏保持原语义不被波及（RFC 3086）
macro_rules! twice {
    ($v:expr_2021) => {
        $v * 2
    };
}

fn reserved_macro_fragment_demo() {
    let result = twice!(21);
    println!("   expr_2021 是合法的片段说明符: twice!(21) = {}", result);

    // 对比：标准 expr 片段（行为与 expr_2021 相同，未来可能演进）
    macro_rules! increment {
        ($v:expr) => {
            $v + 1
        };
    }
    println!("   普通 expr 片段: increment!(41) = {}", increment!(41));
}

// 9. FromIterator 加入 prelude
// Rust 2021 把 TryFrom、TryInto、FromIterator 加入 std prelude：
// HashMap::from_iter 这类由 trait 提供的关联函数无需再手动 use（2015/2018 会 E0599）
fn from_iterator_prelude_demo() {
    use std::collections::HashMap;

    // FromIterator 在 2021 prelude 中，直接调用 from_iter / collect
    let map: HashMap<&str, i32> = HashMap::from_iter([("edition", 2021), ("rust", 1)]);
    println!("   HashMap::from_iter 免导入: {:?}", map);

    let squares: Vec<i32> = (1..=4).map(|n| n * n).collect();
    println!("   collect() 收集为 Vec: {:?}", squares);
}
