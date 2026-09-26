// dyn 关键字示例

// 定义一个trait
trait Animal {
    fn speak(&self);
}

// 实现trait的结构体
struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    // 使用dyn创建trait对象
    let dog: Box<dyn Animal> = Box::new(Dog);
    let cat: Box<dyn Animal> = Box::new(Cat);
    
    // 调用trait方法
    dog.speak();
    cat.speak();
    
    // 存储在向量中
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Dog)
    ];
    
    println!("All animals speak:");
    for animal in animals {
        animal.speak();
    }
}