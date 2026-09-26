// unsafe 关键字示例

// 1. 不安全函数
unsafe fn unsafe_function() {
    println!("This is an unsafe function");
}

// 2. 不安全代码块
fn safe_function() {
    println!("This is a safe function");
    
    // 不安全代码块
    unsafe {
        unsafe_function();
    }
}

// 3. 原始指针操作
fn pointer_operations() {
    let mut x = 5;
    let ptr = &mut x as *mut i32;
    
    unsafe {
        *ptr = 10;
        println!("Modified x through pointer: {}", x);
    }
}

// 4. 访问静态可变变量
static mut COUNTER: i32 = 0;

fn modify_counter() {
    unsafe {
        *(&raw mut COUNTER) += 1;
        println!("Counter: {}", *(&raw mut COUNTER));
    }
}

// 5. 调用外部C函数
unsafe extern "C" {
    fn puts(s: *const u8) -> i32;
}

fn call_c_function() {
    let message = "Hello from C function\0";
    unsafe {
        puts(message.as_ptr());
    }
}

fn main() {
    println!("Using unsafe keyword example");
    
    // 1. 调用安全函数，内部包含不安全代码
    safe_function();
    
    // 2. 指针操作
    pointer_operations();
    
    // 3. 修改静态可变变量
    modify_counter();
    modify_counter();
    
    // 4. 调用外部C函数
    call_c_function();
}