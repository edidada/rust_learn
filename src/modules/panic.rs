// panic 模块示例：标准库中的panic支持
use std::panic;

fn main() {
    // 1. 基本panic
    println!("1. Basic panic:");
    // panic!("This is a panic!"); // 取消注释以测试
    
    // 2. 带格式化信息的panic
    println!("\n2. Formatted panic:");
    // panic!("Value is {} which is invalid", 42); // 取消注释以测试
    
    // 3. 捕获panic
    println!("\n3. Catching panic:");
    let result = panic::catch_unwind(|| {
        println!("Before panic");
        panic!("Something went wrong");
        println!("After panic"); // 不会执行
    });
    
    match result {
        Ok(_) => println!("No panic occurred"),
        Err(panic_info) => println!("Caught panic: {:?}", panic_info),
    }
    
    // 4. 自定义panic钩子
    println!("\n4. Custom panic hook:");
    let original_hook = panic::take_hook();
    
    panic::set_hook(Box::new(|panic_info| {
        println!("Custom panic hook:");
        if let Some(location) = panic_info.location() {
            println!("Panic occurred at {}:{}", location.file(), location.line());
        }
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            println!("Message: {}", message);
        }
    }));
    
    // 测试自定义钩子
    let result = panic::catch_unwind(|| {
        panic!("Test panic for custom hook");
    });
    
    // 恢复原始钩子
    panic::set_hook(original_hook);
    
    // 5. 断言
    println!("\n5. Assertions:");
    let value = 42;
    assert!(value > 0, "Value must be positive");
    assert_eq!(value, 42, "Value must be 42");
    assert_ne!(value, 0, "Value must not be 0");
    
    println!("All assertions passed");
}