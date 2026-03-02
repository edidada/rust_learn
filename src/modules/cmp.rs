// cmp 模块示例：比较和排序
use std::cmp::{max, min, Ordering};

fn main() {
    // 比较两个值
    let a = 10;
    let b = 20;
    
    match a.cmp(&b) {
        Ordering::Less => println!("{} is less than {}", a, b),
        Ordering::Equal => println!("{} is equal to {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
    }
    
    // 使用max和min函数
    println!("Max of {} and {}: {}", a, b, max(a, b));
    println!("Min of {} and {}: {}", a, b, min(a, b));
    
    // 对自定义类型实现PartialOrd和Ord
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let alice = Person { name: "Alice".to_string(), age: 30 };
    let bob = Person { name: "Bob".to_string(), age: 25 };
    let charlie = Person { name: "Charlie".to_string(), age: 35 };
    
    // 比较人员
    println!("Alice < Bob: {}", alice < bob);
    println!("Bob < Charlie: {}", bob < charlie);
    
    // 排序人员
    let mut people = vec![alice, bob, charlie];
    people.sort();
    println!("Sorted people: {:?}", people);
}