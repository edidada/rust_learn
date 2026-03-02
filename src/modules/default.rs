// default 模块示例：Default trait的使用
use std::default::Default;

// 1. 为自定义类型实现Default
// Example 1: 基本使用
#[derive(Debug, Default)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}

// 2. 自定义Default实现
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

// 3. 在泛型中使用Default
trait MyTrait {
    fn do_something(&self);
}

impl<T: Default> MyTrait for T {
    fn do_something(&self) {
        println!("Doing something with {:?}", self);
    }
}

fn main() {
    // 1. 使用派生的Default
    let default_person = Person::default();
    println!("Default person: {:?}", default_person);
    
    // 2. 使用自定义的Default
    let default_point = Point::default();
    println!("Default point: {:?}", default_point);
    
    // 3. 使用Default::default()
    let default_i32: i32 = Default::default();
    let default_string: String = Default::default();
    let default_vec: Vec<i32> = Default::default();
    
    println!("Default i32: {}", default_i32);
    println!("Default string: {:?}", default_string);
    println!("Default vec: {:?}", default_vec);
    
    // 4. 在泛型中使用
    let point = Point { x: 10, y: 20 };
    point.do_something();
}