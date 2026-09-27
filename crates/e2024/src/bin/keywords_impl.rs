// impl 关键字示例

// 1. 为结构体实现方法
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数（静态方法）
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    
    // 实例方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 2. 为类型实现trait
trait Shape {
    fn area(&self) -> u32;
    fn name(&self) -> &str;
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn name(&self) -> &str {
        "Rectangle"
    }
}

// 3. 为trait实现默认方法
trait Printable {
    fn print(&self);
    
    // 默认方法
    fn print_with_prefix(&self, prefix: &str) {
        println!("{}: ", prefix);
        self.print();
    }
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle: {}x{}", self.width, self.height);
    }
}

fn main() {
    // 1. 使用结构体方法
    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
    println!("Is square? {}", rect.is_square());
    
    // 2. 使用trait方法
    let shape: &dyn Shape = &rect;
    println!("Shape area: {}", shape.area());
    println!("Shape name: {}", shape.name());
    
    // 3. 使用带有默认方法的trait
    rect.print();
    rect.print_with_prefix("Details");
}