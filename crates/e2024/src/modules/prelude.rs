// prelude 模块示例：Rust预导入的内容

fn main() {
    // 1. 预导入的类型和特质
    println!("1. Pre-imported types and traits:");
    
    // Option 是预导入的
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    // Result 是预导入的
    let success: Result<i32, String> = Ok(100);
    let error: Result<i32, String> = Err("Error".to_string());
    
    // Vec 是预导入的
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // String 是预导入的
    let string = String::from("Hello");
    
    // 2. 预导入的方法
    println!("\n2. Pre-imported methods:");
    
    // IntoIterator 是预导入的
    let iter = vec.into_iter();
    
    // Drop 是预导入的
    // 当变量离开作用域时会自动调用drop
    
    // 3. 预导入的宏
    println!("\n3. Pre-imported macros:");
    
    // println! 是预导入的
    println!("Hello from println!");
    
    // format! 是预导入的
    let formatted = format!("{} + {} = {}", 1, 2, 3);
    println!("Formatted: {}", formatted);
    
    // panic! 是预导入的
    // panic!("This is a panic"); // 取消注释以测试
    
    // assert! 是预导入的
    assert!(true, "This should pass");
    
    println!("Prelude examples completed");
}