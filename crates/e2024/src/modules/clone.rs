// clone 模块示例：Clone trait的使用
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    // 使用clone方法创建副本
    let person2 = person1.clone();
    
    println!("Original: {:?}", person1);
    println!("Cloned: {:?}", person2);
    
    // 修改克隆后的对象
    let mut person3 = person1.clone();
    person3.name = "Bob".to_string();
    person3.age = 25;
    
    println!("Original after modification: {:?}", person1);
    println!("Modified clone: {:?}", person3);
    
    // 对基本类型使用clone
    let x = 42;
    let y = x.clone();
    println!("x: {}, y: {}", x, y);
    
    // 对字符串使用clone
    let s1 = "Hello".to_string();
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}