// where 关键字示例

// 1. 基本where子句
fn print_items<T>(items: &[T]) where T: std::fmt::Display {
    for item in items {
        println!("{}", item);
    }
}

// 2. 多个约束条件
fn compare_and_print<T, U>(a: T, b: U) where 
    T: std::fmt::Display + PartialOrd,
    U: std::fmt::Display + PartialOrd,
{
    println!("a: {}, b: {}", a, b);
    if a < b {
        println!("a is less than b");
    } else if a > b {
        println!("a is greater than b");
    } else {
        println!("a is equal to b");
    }
}

// 3. 泛型trait约束
struct Wrapper<T>(T);

impl<T> Wrapper<T> where T: std::fmt::Debug {
    fn new(value: T) -> Self {
        Self(value)
    }
    
    fn print(&self) {
        println!("Wrapper contains: {:?}", self.0);
    }
}

// 4. 生命周期约束
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str where 'b: 'a {
    if x.len() > y.len() {
        x
    } else {
        x // 注意：由于生命周期约束，这里只能返回x
    }
}

fn main() {
    println!("Using where keyword example");
    
    // 1. 使用基本where子句
    let numbers = [1, 2, 3, 4, 5];
    println!("Printing numbers:");
    print_items(&numbers);
    
    let strings = ["apple", "banana", "orange"];
    println!("\nPrinting strings:");
    print_items(&strings);
    
    // 2. 使用多个约束条件
    println!("\nComparing numbers:");
    compare_and_print(5, 10);
    
    println!("\nComparing strings:");
    compare_and_print("hello", "world");
    
    // 3. 使用泛型trait约束
    println!("\nUsing wrapper:");
    let wrapper = Wrapper::new(42);
    wrapper.print();
    
    let wrapper_str = Wrapper::new("Hello");
    wrapper_str.print();
    
    // 4. 使用生命周期约束
    println!("\nUsing longest function:");
    let s1 = "hello";
    let s2 = "world";
    let result = longest(s1, s2);
    println!("Longest string: {}", result);
}