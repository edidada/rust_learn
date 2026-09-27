// f32 模块示例：单精度浮点数类型的常量
use std::f32;

fn main() {
    // 1. 基本常量
    println!("1. Basic constants:");
    println!("f32::MIN: {}", f32::MIN);
    println!("f32::MAX: {}", f32::MAX);
    println!("f32::EPSILON: {}", f32::EPSILON);
    println!("f32::INFINITY: {}", f32::INFINITY);
    println!("f32::NEG_INFINITY: {}", f32::NEG_INFINITY);
    println!("f32::NAN: {}", f32::NAN);
    
    // 2. 数学常量
    println!("\n2. Mathematical constants:");
    println!("f32::PI: {}", f32::PI);
    println!("f32::FRAC_PI_2: {}", f32::FRAC_PI_2);
    println!("f32::FRAC_PI_4: {}", f32::FRAC_PI_4);
    println!("f32::FRAC_1_PI: {}", f32::FRAC_1_PI);
    println!("f32::FRAC_2_PI: {}", f32::FRAC_2_PI);
    println!("f32::FRAC_2_SQRT_PI: {}", f32::FRAC_2_SQRT_PI);
    println!("f32::E: {}", f32::E);
    println!("f32::LN_2: {}", f32::LN_2);
    println!("f32::LN_10: {}", f32::LN_10);
    println!("f32::LOG2_E: {}", f32::LOG2_E);
    println!("f32::LOG10_E: {}", f32::LOG10_E);
    println!("f32::SQRT_2: {}", f32::SQRT_2);
    println!("f32::SQRT_1_2: {}", f32::SQRT_1_2);
    
    // 3. 测试特殊值
    println!("\n3. Testing special values:");
    let inf = f32::INFINITY;
    let neg_inf = f32::NEG_INFINITY;
    let nan = f32::NAN;
    
    println!("inf is finite: {}", inf.is_finite());
    println!("inf is infinite: {}", inf.is_infinite());
    println!("nan is NaN: {}", nan.is_nan());
    println!("neg_inf is negative: {}", neg_inf.is_sign_negative());
}