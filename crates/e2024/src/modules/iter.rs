// iter 模块示例：可组合的外部迭代
use std::iter;

fn main() {
    // 1. 基本迭代
    println!("1. Basic iteration:");
    let numbers = vec![1, 2, 3, 4, 5];
    for number in &numbers {
        print!("{} ", number);
    }
    println!();
    
    // 2. 迭代器适配器
    println!("\n2. Iterator adapters:");
    
    // map
    let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // filter
    let even: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even);
    
    // take
    let first_three: Vec<_> = numbers.iter().take(3).collect();
    println!("First three: {:?}", first_three);
    
    // 3. 迭代器组合
    println!("\n3. Iterator composition:");
    let result: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 != 0)
        .map(|&x| x * 3)
        .collect();
    println!("Odd numbers * 3: {:?}", result);
    
    // 4. 特殊迭代器
    println!("\n4. Special iterators:");
    
    // range
    let range: Vec<_> = (1..10).collect();
    println!("Range 1..10: {:?}", range);
    
    // repeat
    let repeated: Vec<_> = iter::repeat(42).take(5).collect();
    println!("Repeated 42: {:?}", repeated);
    
    // zip
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("Zipped: {:?}", zipped);
    
    // 5. 消费者方法
    println!("\n5. Consumer methods:");
    
    // sum
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    // max
    let max = numbers.iter().max();
    println!("Max: {:?}", max);
    
    // fold
    let product: i32 = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);
}