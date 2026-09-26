// become 关键字示例（不稳定特性，需要nightly版本）

// 注意：在稳定版Rust中，become关键字不可用
// 这里使用普通递归和循环代替

// 使用普通递归实现阶乘
fn factorial_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

// 使用循环实现阶乘（更高效）
fn factorial_loop(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    let n = 10;
    
    println!("become keyword example (using alternative implementations)");
    println!("Note: become is an experimental keyword, using regular recursion instead");
    
    let result1 = factorial_recursive(n);
    println!("{}! (recursive) = {}", n, result1);
    
    let result2 = factorial_loop(n);
    println!("{}! (loop) = {}", n, result2);
}