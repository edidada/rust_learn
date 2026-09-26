// trait 关键字示例

// 1. 定义trait
trait Animal {
    fn speak(&self);
    
    // 默认方法
    fn sleep(&self) {
        println!("Zzz...");
    }
}

// 2. 实现trait
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
    
    // 重写默认方法
    fn sleep(&self) {
        println!("Purr...");
    }
}

// 3. trait作为参数
fn make_animal_speak(animal: &impl Animal) {
    animal.speak();
    animal.sleep();
}

// 4. trait作为返回类型
fn create_animal(is_dog: bool) -> Box<dyn Animal> {
    if is_dog {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

// 5. 多trait约束
trait Runnable {
    fn run(&self);
}

impl Runnable for Dog {
    fn run(&self) {
        println!("Dog is running");
    }
}

impl Runnable for Cat {
    fn run(&self) {
        println!("Cat is running");
    }
}

fn animal_activity(animal: &(impl Animal + Runnable)) {
    animal.speak();
    animal.run();
}

fn main() {
    // 1. 使用实现了trait的类型
    let dog = Dog;
    let cat = Cat;
    
    dog.speak();
    dog.sleep();
    
    cat.speak();
    cat.sleep();
    
    // 2. 使用trait作为参数
    println!("\nUsing trait as parameter:");
    make_animal_speak(&dog);
    make_animal_speak(&cat);
    
    // 3. 使用trait作为返回类型
    println!("\nUsing trait as return type:");
    let animal1 = create_animal(true);
    animal1.speak();
    
    let animal2 = create_animal(false);
    animal2.speak();
    
    // 4. 使用多trait约束
    println!("\nUsing multiple trait bounds:");
    animal_activity(&dog);
    animal_activity(&cat);
}