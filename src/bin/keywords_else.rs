// else 关键字示例

fn main() {
    // 1. 基本的if-else
    let number = 5;
    if number > 10 {
        println!("Number is greater than 10");
    } else {
        println!("Number is less than or equal to 10");
    }
    
    // 2. if-else if-else 链
    let score = 85;
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else {
        println!("D");
    }
    
    // 3. else与块表达式
    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("Number is {}", result);
    
    // 4. else与loop结合
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        } else {
            println!("Counter: {}", counter);
        }
    }
}