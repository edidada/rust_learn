//! Rust 2018 Edition 特性说明
//!
//! Rust 2018 Edition于2018年12月发布，带来了许多语法改进和新特性。

fn main() {
    println!("=== Rust 2018 Edition 新增特性 ===\n");

    // 1. 模块系统改进
    println!("1. 模块系统改进 (Module System)");
    module_system_demo();

    // 2. async/await (预览)
    println!("\n2. async/await 关键字引入");
    async_demo();

    // 3. dyn Trait
    println!("\n3. dyn Trait 显式动态分发");
    dyn_trait_demo();

    // 4. 匿名生命周期 '_
    println!("\n4. 匿名生命周期 '_");
    anonymous_lifetime_demo();

    // 5. 原始标识符 raw identifiers
    println!("\n5. 原始标识符 (raw identifiers)");
    raw_identifier_demo();

    // 6. 切片模式匹配
    println!("\n6. 切片模式匹配");
    slice_patterns_demo();

    // 7. 函数指针和闭包改进
    println!("\n7. 函数指针改进");
    function_pointer_demo();

    // 8. 其他改进（NLL 非词法生命周期等）
    println!("\n8. 其他改进 (NLL 非词法生命周期)");
    other_improvements();

    // 9. 嵌套分组导入与路径式宏导入
    println!("\n9. 嵌套分组导入与宏导入 (Grouped & Macro Imports)");
    grouped_import_demo();
}

// 1. 模块系统改进
// Rust 2018简化了模块导入，不再需要extern crate
fn module_system_demo() {
    // 可以直接使用crate::前缀
    println!("   - 使用crate::前缀直接访问crate根");
    println!("   - 不再需要extern crate声明");
    println!("   - 统一的路径语法");
}

// 2. async/await
// Rust 2018引入了async/await关键字（虽然稳定版稍晚）
fn async_demo() {
    println!("   async fn - 定义异步函数");
    println!("   .await - 等待异步操作完成（后缀语法，不是宏）");
    println!("   为异步编程提供语法糖");

    // 示例代码（需要async运行时）
    /*
    async fn fetch_data() -> String {
        // 异步操作
        "data".to_string()
    }

    async fn main_async() {
        let data = fetch_data().await;
        println!("{}", data);
    }
    */
}

// 3. dyn Trait 显式动态分发
trait Drawable {
    fn draw(&self);
}

struct Circle;
impl Drawable for Circle {
    fn draw(&self) {
        println!("   绘制圆形");
    }
}

fn dyn_trait_demo() {
    // Rust 2018之前
    // let obj: Box<Drawable> = Box::new(Circle);

    // Rust 2018之后，必须使用dyn
    let obj: Box<dyn Drawable> = Box::new(Circle);
    obj.draw();

    println!("   dyn关键字使动态分发显式化");
    println!("   提高了代码可读性");
}

// 4. 匿名生命周期 '_
fn anonymous_lifetime_demo() {
    // 在Rust 2018中，'_ 可以用作匿名生命周期
    fn _foo(_: &'_ str) -> &'_ str {
        ""
    }

    // 结构体定义仍需显式命名生命周期（'_ 不能作为结构体参数名），
    // 但 impl 块可用 '_ 省略：
    struct _Parser<'a> {
        input: &'a str,
    }
    impl _Parser<'_> {
        fn peek(&self) -> &str {
            self.input
        }
    }
    let parser = _Parser { input: "hello" };
    println!("   impl _Parser<'_> 的 peek(): {}", parser.peek());

    println!("   '_ 用于省略显式生命周期参数");
    println!("   简化生命周期标注");
}

// 5. 原始标识符
fn raw_identifier_demo() {
    // 使用r#前缀可以使用Rust关键字作为标识符
    let r#match = "可以使用match作为变量名";
    println!("   r#match = {}", r#match);

    // 这在与其他语言交互时特别有用
    // 例如C库中有名为match的函数
    println!("   r#前缀允许使用关键字作为标识符");
}

// 6. 切片模式匹配
fn slice_patterns_demo() {
    let arr = [1, 2, 3, 4, 5];

    match &arr[..] {
        [first, second, ..] => {
            println!("   前两个元素: {}, {}", first, second);
        }
        [only] => println!("   只有一个元素: {}", only),
        [] => println!("   空数组"),
    }

    // 使用@绑定
    match &arr[..] {
        [first @ 1, ..] => println!("   以1开头: {}", first),
        _ => println!("   其他"),
    }
}

// 7. 函数指针改进
fn function_pointer_demo() {
    // Rust 2018统一了函数指针和闭包的类型
    fn _add_one(x: i32) -> i32 {
        x + 1
    }

    // 函数指针类型
    let _f: fn(i32) -> i32 = |x| x + 1;

    println!("   函数指针和闭包类型更加统一");
    println!("   改进了Fn trait的自动实现");
}

// 其他Rust 2018改进
fn other_improvements() {
    // 1. 非词法生命周期 (NLL)
    // 改进了借用检查器，允许更多有效的代码
    let mut x = 5;
    let y = &x;
    println!("   y 引用 x: {}", y);
    // 这里y不再被使用，所以可以改变x
    x = 6; // 在Rust 2015中这会报错（NLL之前借用需持续到作用域结束）
    println!("   NLL: y 不再使用后修改 x 成功, x = {}", x);

    // 2. 统一路径
    // 统一了use语句中的路径语法

    // 3. 常量泛型（预览）
    // 允许在泛型中使用常量值
}

// 9. 嵌套分组导入与路径式宏导入
// Rust 2018：use 支持 {} 分组；宏可像函数一样按路径 use 导入（1.30+），
// 取代 2015 的 #[macro_use] extern crate 惯例
fn grouped_import_demo() {
    // 分组导入：一个 use 引入同一路径下的多个名字
    use std::collections::{BTreeMap, HashMap, HashSet};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 2018);
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("rust");
    let btree: BTreeMap<&str, i32> = BTreeMap::new();
    println!(
        "   分组导入 HashMap/HashSet/BTreeMap: {} {} {}",
        map.len(),
        set.len(),
        btree.len()
    );

    // 路径式宏导入：直接 use 宏的定义路径
    use std::dbg;
    let version = 2018;
    dbg!(version);
    println!("   use std::dbg 导入宏并调用成功（dbg 输出见上方 stderr）");
}
