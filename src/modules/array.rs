// array 模块示例：数组操作
use std::array;

fn main() {
    // 创建数组
    let arr = [1, 2, 3, 4, 5];
    
    // 数组长度
    println!("Array length: {}", arr.len());
    
    // 访问元素
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[arr.len() - 1]);
    
    // 数组迭代
    println!("Array elements:");
    for elem in &arr {
        print!("{} ", elem);
    }
    println!();
    
    // 使用array::from_fn创建数组
    let squares = array::from_fn(|i| i * i);
    println!("Squares: {:?}", squares);
    
    // 数组分割
    let (first, rest) = arr.split_first().unwrap();
    println!("First: {}, Rest: {:?}", first, rest);
    
    let (prefix, suffix) = arr.split_at(2);
    println!("Prefix: {:?}, Suffix: {:?}", prefix, suffix);
}