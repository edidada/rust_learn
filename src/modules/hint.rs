// hint 模块示例：影响代码发射或优化的编译器提示
use std::hint;

fn main() {
    // 1. 告诉编译器一个条件很可能为真
    println!("1. Using likely:");
    let x = 5;
    if hint::likely(x == 5) {
        println!("x is likely 5");
    } else {
        println!("x is not 5");
    }
    
    // 2. 告诉编译器一个条件很可能为假
    println!("\n2. Using unlikely:");
    let y = 10;
    if hint::unlikely(y == 0) {
        println!("y is unlikely 0");
    } else {
        println!("y is not 0");
    }
    
    // 3. 告诉编译器不要优化掉一个循环
    println!("\n3. Using black_box:");
    let mut counter = 0;
    for i in 0..1000 {
        counter += i;
        // 防止编译器优化掉循环
        hint::black_box(counter);
    }
    println!("Counter: {}", counter);
    
    // 4. 告诉编译器一个值可能已经改变（用于unsafe代码）
    println!("\n4. Using assume:");
    let mut z = 10;
    // 告诉编译器z的值现在是20
    unsafe {
        hint::assume(z == 20);
    }
    println!("z: {}", z);
}