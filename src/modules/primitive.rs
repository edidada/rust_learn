// primitive 模块示例：重新导出原始类型
use std::primitive;

fn main() {
    // 1. 原始类型
    println!("1. Primitive types:");
    
    // 整数类型
    let i8_value: i8 = 127;
    let i16_value: i16 = 32767;
    let i32_value: i32 = 2147483647;
    let i64_value: i64 = 9223372036854775807;
    let i128_value: i128 = 170141183460469231731687303715884105727;
    let isize_value: isize = isize::MAX;
    
    // 无符号整数类型
    let u8_value: u8 = 255;
    let u16_value: u16 = 65535;
    let u32_value: u32 = 4294967295;
    let u64_value: u64 = 18446744073709551615;
    let u128_value: u128 = 340282366920938463463374607431768211455;
    let usize_value: usize = usize::MAX;
    
    // 浮点类型
    let f32_value: f32 = 3.14;
    let f64_value: f64 = 3.141592653589793;
    
    // 布尔类型
    let bool_value: bool = true;
    
    // 字符类型
    let char_value: char = 'A';
    
    // 2. 打印原始类型的值
    println!("\n2. Printing primitive values:");
    println!("i8: {}", i8_value);
    println!("i32: {}", i32_value);
    println!("u8: {}", u8_value);
    println!("u32: {}", u32_value);
    println!("f32: {}", f32_value);
    println!("f64: {}", f64_value);
    println!("bool: {}", bool_value);
    println!("char: {}", char_value);
    
    // 3. 原始类型的大小
    println!("\n3. Size of primitive types:");
    println!("Size of i8: {} bytes", std::mem::size_of::<i8>());
    println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("Size of i64: {} bytes", std::mem::size_of::<i64>());
    println!("Size of f32: {} bytes", std::mem::size_of::<f32>());
    println!("Size of f64: {} bytes", std::mem::size_of::<f64>());
    println!("Size of bool: {} bytes", std::mem::size_of::<bool>());
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}