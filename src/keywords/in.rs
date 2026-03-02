// in 关键字示例

fn main() {
    // 1. 在数组上迭代
    let numbers = [1, 2, 3, 4, 5];
    println!("1. Iterating over array:");
    for number in numbers {
        print!("{} ", number);
    }
    println!();
    
    // 2. 在范围上迭代
    println!("2. Iterating over range:");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    
    // 3. 在字符串上迭代字符
    let message = "Hello";
    println!("3. Iterating over string characters:");
    for c in message.chars() {
        print!("{} ", c);
    }
    println!();
    
    // 4. 在向量上迭代
    let fruits = vec!["apple", "banana", "orange"];
    println!("4. Iterating over vector:");
    for fruit in fruits {
        print!("{} ", fruit);
    }
    println!();
    
    // 5. 在迭代器上迭代
    println!("5. Iterating over iterator:");
    for number in (0..10).step_by(2) {
        print!("{} ", number);
    }
    println!();
}