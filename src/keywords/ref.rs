// ref 关键字示例

fn main() {
    // 1. 基本的ref绑定
    let value = 42;
    let ref ref_value = value;
    println!("value: {}, ref_value: {}", value, *ref_value);
    
    // 2. 在match语句中使用ref
    let optional_value: Option<String> = Some("Hello".to_string());
    match optional_value {
        Some(ref s) => println!("Got a string: {}", s),
        None => println!("Got nothing"),
    }
    
    // 注意：使用ref后，原始值仍然存在
    if let Some(ref s) = optional_value {
        println!("Again, got a string: {}", s);
    }
    
    // 3. 使用ref mut进行可变引用
    let mut mutable_value = 100;
    let ref mut ref_mut_value = mutable_value;
    *ref_mut_value = 200;
    println!("mutable_value: {}", mutable_value);
    
    // 4. 在结构体模式中使用ref
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let Person { ref name, ref age } = person;
    println!("Person: {} ({})
", name, age);
}