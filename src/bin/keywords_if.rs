// if 关键字示例

fn main() {
    // 1. 基本if语句
    let number = 5;
    if number > 0 {
        println!("Number is positive");
    }
    
    // 2. if-else语句
    let temperature = 25;
    if temperature > 30 {
        println!("It's hot outside");
    } else {
        println!("It's not too hot");
    }
    
    // 3. if-else if-else链
    let score = 75;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "D"
    };
    println!("Grade: {}", grade);
    
    // 4. 作为表达式
    let is_even = if number % 2 == 0 {
        true
    } else {
        false
    };
    println!("Is number even? {}", is_even);
    
    // 5. 与let绑定结合
    let result = if let Some(value) = Some(42) {
        value
    } else {
        0
    };
    println!("Result: {}", result);
}