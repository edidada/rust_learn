// struct 关键字示例

// 1. 基本结构体
struct Person {
    name: String,
    age: u32,
}

// 2. 元组结构体
struct Point(i32, i32);

// 3. 单元结构体
struct Unit;

// 4. 带方法的结构体
impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
    
    fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old", self.name, self.age);
    }
}

// 为元组结构体实现方法
impl Point {
    fn distance_from_origin(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
    }
}

fn main() {
    // 1. 使用基本结构体
    let person = Person::new("Alice", 30);
    person.greet();
    
    // 2. 使用元组结构体
    let point = Point(3, 4);
    println!("Point: ({}, {})", point.0, point.1);
    println!("Distance from origin: {}", point.distance_from_origin());
    
    // 3. 使用单元结构体
    let unit = Unit;
    println!("Unit struct created");
}