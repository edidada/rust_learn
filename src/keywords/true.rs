// true 关键字示例

fn main() {
    // 1. 基本布尔值
    let is_true: bool = true;
    println!("is_true: {}", is_true);
    
    // 2. 在条件语句中使用
    if true {
        println!("This will be printed because condition is true");
    } else {
        println!("This won't be printed");
    }
    
    // 3. 布尔运算
    let a = true;
    let b = false;
    println!("a && b: {}", a && b);
    println!("a || b: {}", a || b);
    println!("!a: {}", !a);
    
    // 4. 与其他类型的转换
    let int_true: i32 = true as i32;
    println!("true as i32: {}", int_true);
    
    // 5. 在匹配语句中使用
    match is_true {
        true => println!("It's true"),
        false => println!("It's false"),
    }
}