// convert 模块示例：类型转换
use std::convert::{From, Into, TryFrom, TryInto};

// 自定义类型
#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

// 实现From trait
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// 实现TryFrom trait
impl TryFrom<Fahrenheit> for Celsius {
    type Error = &'static str;
    
    fn try_from(f: Fahrenheit) -> Result<Self, Self::Error> {
        if f.0 < -459.67 {
            Err("Temperature below absolute zero")
        } else {
            Ok(Celsius((f.0 - 32.0) * 5.0 / 9.0))
        }
    }
}

fn main() {
    // 使用From trait
    let celsius = Celsius(25.0);
    let fahrenheit: Fahrenheit = celsius.into();
    println!("25°C = {:?}°F", fahrenheit.0);
    
    // 使用TryFrom trait
    let fahrenheit = Fahrenheit(77.0);
    let celsius: Result<Celsius, _> = fahrenheit.try_into();
    match celsius {
        Ok(c) => println!("77°F = {:?}°C", c.0),
        Err(e) => println!("Error: {}", e),
    }
    
    // 标准类型转换
    let s = "123";
    let n: i32 = s.parse().unwrap();
    println!("String to i32: {}", n);
    
    let n = 456;
    let s: String = n.to_string();
    println!("i32 to String: {}", s);
}