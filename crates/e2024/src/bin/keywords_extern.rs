#![allow(dead_code)]
// extern 关键字示例

// 1. 声明外部函数（C语言风格）
unsafe extern "C" {
    fn puts(s: *const u8) -> i32;
    fn getchar() -> i32;
}

// 2. 导入外部crate
// 注意：在实际项目中，需要在Cargo.toml中添加依赖
// extern crate rand;

// 3. 定义外部块，供其他语言调用
#[unsafe(no_mangle)]
pub extern "C" fn rust_function(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Using extern keyword example");
    
    // 调用外部C函数
    unsafe {
        let message = "Hello from extern C function\0";
        puts(message.as_ptr());
        println!("Press any key to continue...");
        // getchar(); // 取消注释以等待用户输入
    }
    
    // 调用我们定义的外部函数
    let result = rust_function(5, 7);
    println!("rust_function(5, 7) = {}", result);
}