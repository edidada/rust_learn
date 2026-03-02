// fn 关键字示例

// 1. 基本函数定义
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 2. 无返回值函数
fn print_hello() {
    println!("Hello!");
}

// 3. 函数指针类型
fn main() {
    // 调用基本函数
    let result = add(5, 7);
    println!("5 + 7 = {}", result);
    
    // 调用无返回值函数
    print_hello();
    
    // 使用函数指针
    let add_ptr: fn(i32, i32) -> i32 = add;
    let ptr_result = add_ptr(10, 20);
    println!("Using function pointer: 10 + 20 = {}", ptr_result);
    
    // 函数作为参数
    fn apply_operation(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
        op(a, b)
    }
    
    let sum = apply_operation(3, 4, add);
    println!("apply_operation(3, 4, add) = {}", sum);
}