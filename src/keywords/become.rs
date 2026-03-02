// become 关键字示例（不稳定特性，需要nightly版本）

#![feature(tail_call)]

// 使用become进行尾递归
fn factorial(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        // 尾递归调用
        become factorial(n - 1, acc * n);
    }
}

fn main() {
    let n = 10;
    let result = factorial(n, 1);
    println!("{}! = {}", n, result);
}