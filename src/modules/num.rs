// num 模块示例：数字的附加功能
use std::num;

fn main() {
    // 1. 整数类型的边界值
    println!("1. Integer bounds:");
    println!("i8::MIN: {}", i8::MIN);
    println!("i8::MAX: {}", i8::MAX);
    println!("u8::MIN: {}", u8::MIN);
    println!("u8::MAX: {}", u8::MAX);
    println!("i32::MIN: {}", i32::MIN);
    println!("i32::MAX: {}", i32::MAX);
    println!("u32::MIN: {}", u32::MIN);
    println!("u32::MAX: {}", u32::MAX);
    
    // 2. 浮点数特殊值
    println!("\n2. Floating point special values:");
    println!("f32::INFINITY: {}", f32::INFINITY);
    println!("f32::NEG_INFINITY: {}", f32::NEG_INFINITY);
    println!("f32::NAN: {}", f32::NAN);
    
    // 3. 数字转换
    println!("\n3. Numeric conversions:");
    let x: i32 = 42;
    let y: u64 = x as u64;
    println!("i32 to u64: {} -> {}", x, y);
    
    let a: f32 = 3.14;
    let b: i32 = a as i32;
    println!("f32 to i32: {} -> {}", a, b);
    
    // 4. 溢出处理
    println!("\n4. Overflow handling:");
    let overflow = i8::MAX.wrapping_add(1);
    println!("i8::MAX + 1 (wrapping): {}", overflow);
    
    // 5. 数字解析
    println!("\n5. Numeric parsing:");
    let parsed: Result<i32, _> = "123".parse();
    println!("Parsing '123': {:?}", parsed);
    
    let parsed_float: Result<f64, _> = "3.14".parse();
    println!("Parsing '3.14': {:?}", parsed_float);
    
    // 6. 格式化数字
    println!("\n6. Formatting numbers:");
    println!("Decimal: {}", 42);
    println!("Hexadecimal: {:x}", 42);
    println!("Octal: {:o}", 42);
    println!("Binary: {:b}", 42);
    println!("Float: {:.2}", 3.14159);
}