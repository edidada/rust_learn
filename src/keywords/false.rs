// false 关键字示例

fn main() {
    // 1. 基本布尔值
    let is_false: bool = false;
    println!("is_false: {}", is_false);
    
    // 2. 在条件语句中使用
    if false {
        println!("This won't be printed");
    } else {
        println!("This will be printed because condition is false");
    }
    
    // 3. 布尔运算
    let a = true;
    let b = false;
    println!("a && b: {}", a && b);
    println!("a || b: {}", a || b);
    println!("!b: {}", !b);
    
    // 4. 与其他类型的转换
    let int_false: i32 = false as i32;
    println!("false as i32: {}", int_false);
    
    // 5. 在匹配语句中使用
    match is_false {
        true => println!("It's true"),
        false => println!("It's false"),
    }
}