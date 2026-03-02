// boxed 模块示例：Box<T>类型的堆分配
use std::boxed::Box;

fn main() {
    // 在堆上分配单个值
    let boxed_i32 = Box::new(42);
    println!("Boxed i32: {}", boxed_i32);
    
    // 在堆上分配结构体
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let boxed_person = Box::new(Person {
        name: "Alice".to_string(),
        age: 30,
    });
    println!("Boxed person: {:?}", boxed_person);
    
    // 解箱（获取所有权）
    let person = *boxed_person;
    println!("Unboxed person: {:?}", person);
    
    // Box::leak 示例（泄漏内存，返回永久引用）
    let static_str: &'static str = Box::leak("Hello".to_string().into_boxed_str());
    println!("Static string: {}", static_str);
}