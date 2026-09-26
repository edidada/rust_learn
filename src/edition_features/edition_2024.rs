//! Rust 2024 Edition 特性说明
//!
//! Rust 2024 Edition于2024年发布，带来了许多现代化的语法改进和新特性。

fn main() {
    println!("=== Rust 2024 Edition 新增特性 ===\n");

    // 1. 临时作用域延长
    println!("1. 临时作用域延长 (Temporary Lifetime Extension)");
    temporary_lifetime_extension();

    // 2. 宏改进
    println!("\n2. 宏系统改进 (Macro Improvements)");
    macro_improvements();

    // 3. 模式匹配改进
    println!("\n3. 模式匹配改进 (Pattern Matching)");
    pattern_matching_improvements();

    // 4. 类型推断改进
    println!("\n4. 类型推断改进 (Type Inference)");
    type_inference_improvements();

    // 5. 新的标准库特性
    println!("\n5. 标准库新特性 (Standard Library)");
    std_library_features();

    // 6. Cargo改进
    println!("\n6. Cargo工具改进");
    cargo_improvements();

    // 7. 异步编程改进
    println!("\n7. 异步编程改进 (Async)");
    async_improvements();

    // 8. if let 临时值作用域收紧
    println!("\n8. if let 临时值作用域收紧 (if let Temporary Scope)");
    if_let_temp_scope_demo();

    // 9. 尾表达式临时值作用域收紧
    println!("\n9. 尾表达式临时值作用域收紧 (Tail Expression Temporary Scope)");
    tail_expr_temp_scope_demo();
}

// 1. 临时作用域延长
fn temporary_lifetime_extension() {
    // Rust 2024改进了临时值的生命周期
    // 使得更多代码模式可以编译通过

    // 示例：更灵活的借用
    let data = vec![1, 2, 3];
    let _first = &data[0]; // 在Rust 2024中生命周期更灵活

    println!("   临时值生命周期更加灵活");
    println!("   减少了不必要的生命周期错误");
    println!("   允许更多自然的代码模式");
}

// 2. 宏系统改进
fn macro_improvements() {
    // Rust 2024改进了宏系统的几个方面

    // 更清晰的宏错误信息
    println!("   改进的宏错误诊断");
    println!("   更好的宏扩展调试支持");
    println!("   过程宏的改进");

    // 声明宏的改进
    macro_rules! improved_macro {
        // 更灵活的模式匹配
        ($($x:expr),+ $(,)?) => {
            [$($x),+]
        };
    }

    let arr = improved_macro![1, 2, 3, 4, 5];
    println!("   宏创建的数组: {:?}", arr);
}

// 3. 模式匹配改进
fn pattern_matching_improvements() {
    // Rust 2024带来了模式匹配的改进

    // 更灵活的匹配守卫
    let x = Some(5);
    match x {
        Some(n) if n > 0 && n < 10 => {
            println!("   匹配到0-10之间的数: {}", n);
        }
        _ => println!("   其他"),
    }

    // 嵌套解构改进
    let point = (Some(1), Some(2));
    match point {
        (Some(x), Some(y)) => {
            println!("   点坐标: ({}, {})", x, y);
        }
        _ => println!("   无效坐标"),
    }
}

// 4. 类型推断改进
fn type_inference_improvements() {
    // Rust 2024改进了类型推断

    // 更智能的泛型推断
    fn process<T: Default>(_: T) -> T {
        T::default()
    }

    let _result: i32 = process(42);

    // 闭包类型推断改进
    let closure = |x| x + 1;
    let _result = closure(5);

    println!("   改进的泛型类型推断");
    println!("   更好的闭包类型推断");
    println!("   减少显式类型标注的需要");
}

// 5. 标准库新特性
fn std_library_features() {
    // Rust 2024标准库的新增内容

    // 新的Iterator方法
    let nums = vec![1, 2, 3, 4, 5];

    // 使用新的迭代器方法
    let sum: i32 = nums.iter().sum();
    println!("   迭代器求和: {}", sum);

    // 改进的Option/Result方法
    let opt: Option<i32> = Some(42);
    let _ = opt.inspect(|v| println!("   Option值: {}", v));

    // 新的字符串方法
    let s = "Hello, Rust 2024!";
    println!("   字符串: {}", s);

    println!("   新增Iterator方法");
    println!("   改进的Option/Result API");
    println!("   更多实用的字符串方法");
}

// 6. Cargo改进
fn cargo_improvements() {
    println!("   更快的编译速度");
    println!("   改进的依赖解析");
    println!("   更好的构建缓存");
    println!("   增强的诊断信息");

    // Cargo.toml的新特性
    println!("   支持新的manifest格式");
    println!("   改进的workspace支持");
}

// 7. 异步编程改进
fn async_improvements() {
    // Rust 2024对async/await的改进

    println!("   改进的异步trait支持");
    println!("   更好的异步闭包");
    println!("   异步迭代器(AsyncIterator)");

    // async fn示例
    async fn fetch_data() -> String {
        // 模拟异步操作
        "数据".to_string()
    }

    // 使用async块
    let _future = async {
        let data = fetch_data().await;
        println!("   获取到: {}", data);
    };

    println!("   更稳定的异步生态系统");
    println!("   改进的异步运行时支持");
}

// 8. if let 临时值作用域收紧
// Rust 2024 收紧了 if let 和尾表达式等场景下临时值的生命周期。
// if let：scrutinee 中生成的临时值，2021 及之前存活到整个 if let 表达式
// （含 else 分支）结束；2024 收紧为"进入 else 分支前就 drop"
// （then 分支行为不变）。这直接影响了借用检查的结果。
fn if_let_temp_scope_demo() {
    use std::sync::RwLock;

    // 演示 1：经典 RwLock 死锁场景
    let value = RwLock::new(None::<i32>);
    if let Some(x) = *value.read().unwrap() {
        println!("   then 分支读到: {}", x);
    } else {
        // 2024：读锁临时值在进入 else 前已释放 → 这里能拿到写锁
        // 2021：读锁仍被临时值持有 → write() 永久阻塞（死锁）
        let mut v = value.write().unwrap();
        *v = Some(1);
        println!("   else 分支成功拿到写锁（同一段代码在 2021 会死锁）");
    }

    // 演示 2：用自定义 Drop 观察临时值 drop 时机
    println!("   2024 输出顺序: [make] → [drop] → else 分支");
    println!("   2021 输出顺序: [make] → else 分支 → [drop]");
    if let None = Some(LogGuard::new("if-let 临时值")) {
        println!("   then 分支");
    } else {
        println!("   else 分支");
    }
}

// 9. 尾表达式临时值作用域收紧
// 2021 及之前：尾表达式中的临时值会扩展到块外（在局部变量之后才 drop）；
// 2024：尾表达式临时值在块末尾立即 drop（先于局部变量），作用域被收紧。
fn tail_expr_temp_scope_demo() {
    // 2021：error[E0597]: `c` does not live long enough
    //       （临时 Ref 比局部变量 c 活得久）
    // 2024：临时值先于局部变量 drop → 编译通过
    println!("   c.borrow().len() = {}", tail_borrow_len());
}

fn tail_borrow_len() -> usize {
    let c = std::cell::RefCell::new("..");
    c.borrow().len() // 2024：尾表达式临时值在此处立即 drop
}

// 观察临时值 drop 时机的辅助类型
struct LogGuard(&'static str);

impl LogGuard {
    fn new(tag: &'static str) -> LogGuard {
        println!("   [make] {}", tag);
        LogGuard(tag)
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        println!("   [drop] {}", self.0);
    }
}

// 其他Rust 2024特性
#[allow(dead_code)]
fn other_features() {
    // 1. 编译器诊断改进
    // 更清晰、更有帮助的错误信息

    // 2. 性能优化
    // 更好的代码生成
    // 改进的优化器

    // 3. 安全性改进
    // 更强的安全检查
    // 改进的不安全代码指导

    // 4. 工具链改进
    // rustfmt改进
    // clippy新lint
    // rust-analyzer增强

    println!("其他特性:");
    println!("- 改进的编译器诊断");
    println!("- 性能优化");
    println!("- 安全性增强");
    println!("- 开发工具改进");
}
