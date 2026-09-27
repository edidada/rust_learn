// ffi 模块示例：与FFI绑定相关的工具
use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::raw::c_char;

// 1. C字符串与Rust字符串的转换
fn c_string_example() {
    println!("1. C string examples:");
    
    // Rust字符串转C字符串
    let rust_str = "Hello from Rust";
    let c_string = CString::new(rust_str).expect("CString::new failed");
    println!("Rust string: {}", rust_str);
    
    // C字符串转Rust字符串
    let c_str = CStr::from_ptr(c_string.as_ptr());
    let rust_str_from_c = c_str.to_str().expect("CStr::to_str failed");
    println!("Rust string from C: {}", rust_str_from_c);
}

// 2. 操作系统字符串
fn os_string_example() {
    println!("\n2. OS string examples:");
    
    // Rust字符串转OsString
    let rust_str = "Path/to/file";
    let os_string = OsString::from(rust_str);
    println!("Rust string: {}", rust_str);
    println!("OsString: {:?}", os_string);
    
    // OsString转Rust字符串
    if let Some(rust_str_from_os) = os_string.to_str() {
        println!("Rust string from OsString: {}", rust_str_from_os);
    }
    
    // OsStr示例
    let os_str = OsStr::new("OsStr example");
    println!("OsStr: {:?}", os_str);
    if let Some(rust_str) = os_str.to_str() {
        println!("Rust string from OsStr: {}", rust_str);
    }
}

// 3. 模拟调用C函数
// 注意：实际使用时需要链接相应的C库
// extern "C" {
//     fn puts(s: *const c_char) -> i32;
// }

fn main() {
    c_string_example();
    os_string_example();
    
    // 调用C函数示例（注释掉，因为没有链接C库）
    // let message = CString::new("Hello from Rust to C").expect("CString::new failed");
    // unsafe {
    //     puts(message.as_ptr());
    // }
}