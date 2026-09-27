// mem 模块示例：处理内存的基本函数
use std::mem;

fn main() {
    // 1. 大小计算
    println!("1. Size calculations:");
    println!("Size of i32: {} bytes", mem::size_of::<i32>());
    println!("Size of f64: {} bytes", mem::size_of::<f64>());
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    
    // 2. 对齐计算
    println!("\n2. Alignment calculations:");
    println!("Alignment of i32: {} bytes", mem::align_of::<i32>());
    println!("Alignment of f64: {} bytes", mem::align_of::<f64>());
    
    // 3. 内存复制
    println!("\n3. Memory copy:");
    let mut src = [1, 2, 3, 4, 5];
    let mut dst = [0; 5];
    
    println!("Before copy: src={:?}, dst={:?}", src, dst);
    mem::copy(&src, &mut dst);
    println!("After copy: src={:?}, dst={:?}", src, dst);
    
    // 4. 内存交换
    println!("\n4. Memory swap:");
    let mut a = 10;
    let mut b = 20;
    
    println!("Before swap: a={}, b={}", a, b);
    mem::swap(&mut a, &mut b);
    println!("After swap: a={}, b={}", a, b);
    
    // 5. 零初始化
    println!("\n5. Zero initialization:");
    let mut zeroed: [i32; 5] = mem::zeroed();
    println!("Zeroed array: {:?}", zeroed);
    
    // 6. 替换值
    println!("\n6. Replace value:");
    let mut value = 42;
    let old_value = mem::replace(&mut value, 100);
    println!("Old value: {}, New value: {}", old_value, value);
    
    // 7. 取最小值
    println!("\n7. Minimum value:");
    let min_i32 = mem::min(10, 20);
    println!("min(10, 20): {}", min_i32);
    
    // 8. 取最大值
    println!("\n8. Maximum value:");
    let max_i32 = mem::max(10, 20);
    println!("max(10, 20): {}", max_i32);
}