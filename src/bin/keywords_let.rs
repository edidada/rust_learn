// let 关键字示例

fn main() {
    // 1. 基本变量绑定
    let x = 5;
    println!("x = {}", x);
    
    // 2. 可变变量
    let mut y = 10;
    println!("Initial y = {}", y);
    y = 20;
    println!("Updated y = {}", y);
    
    // 3. 类型标注
    let z: i32 = 15;
    println!("z = {} (type: i32)", z);
    
    // 4. 模式匹配绑定
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);
    
    // 5. 忽略值
    let (c, _) = (3, 4);
    println!("c = {}", c);
    
    // 6. 绑定到表达式
    let result = if x > y {
        "x is greater"
    } else {
        "y is greater"
    };
    println!("Result: {}", result);
    
    // 7. 阴影变量
    let x = 100;
    println!("Shadowed x = {}", x);
}