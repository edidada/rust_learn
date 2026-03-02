// f64 模块示例：双精度浮点数类型的常量
use std::f64;

fn main() {
    // 1. 基本常量
    println!("1. Basic constants:");
    println!("f64::MIN: {}", f64::MIN);
    println!("f64::MAX: {}", f64::MAX);
    println!("f64::EPSILON: {}", f64::EPSILON);
    println!("f64::INFINITY: {}", f64::INFINITY);
    println!("f64::NEG_INFINITY: {}", f64::NEG_INFINITY);
    println!("f64::NAN: {}", f64::NAN);
    
    // 2. 数学常量
    println!("\n2. Mathematical constants:");
    println!("f64::PI: {}", f64::PI);
    println!("f64::FRAC_PI_2: {}", f64::FRAC_PI_2);
    println!("f64::FRAC_PI_4: {}", f64::FRAC_PI_4);
    println!("f64::FRAC_1_PI: {}", f64::FRAC_1_PI);
    println!("f64::FRAC_2_PI: {}", f64::FRAC_2_PI);
    println!("f64::FRAC_2_SQRT_PI: {}", f64::FRAC_2_SQRT_PI);
    println!("f64::E: {}", f64::E);
    println!("f64::LN_2: {}", f64::LN_2);
    println!("f64::LN_10: {}", f64::LN_10);
    println!("f64::LOG2_E: {}", f64::LOG2_E);
    println!("f64::LOG10_E: {}", f64::LOG10_E);
    println!("f64::SQRT_2: {}", f64::SQRT_2);
    println!("f64::SQRT_1_2: {}", f64::SQRT_1_2);
    
    // 3. 测试特殊值
    println!("\n3. Testing special values:");
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    println!("inf is finite: {}", inf.is_finite());
    println!("inf is infinite: {}", inf.is_infinite());
    println!("nan is NaN: {}", nan.is_nan());
    println!("neg_inf is negative: {}", neg_inf.is_sign_negative());
}