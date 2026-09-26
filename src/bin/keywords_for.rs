// for 关键字示例

// 1. 迭代用法
fn iterate_example() {
    println!("1. Iteration with for:");
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers {
        print!("{} ", number);
    }
    println!();
    
    // 迭代范围
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
}

// 2. trait实现用法
trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
}

// 使用for实现trait
impl Printable for Person {
    fn print(&self) {
        println!("Person: {}", self.name);
    }
}

// 3. 高阶trait边界
fn print_all<T>(items: &[T]) where for<'a> &'a T: Printable {
    for item in items {
        item.print();
    }
}

// 为引用实现Printable
impl<'a> Printable for &'a Person {
    fn print(&self) {
        println!("Person reference: {}", self.name);
    }
}

fn main() {
    // 1. 迭代示例
    iterate_example();
    
    // 2. trait实现示例
    let person = Person { name: "Alice".to_string() };
    person.print();
    
    // 3. 高阶trait边界示例
    let people = vec![
        Person { name: "Bob".to_string() },
        Person { name: "Charlie".to_string() }
    ];
    print_all(&people);
}