// type 关键字示例

// 1. 基本类型别名
type MyInt = i32;
type StringResult = Result<String, Box<dyn std::error::Error>>;

// 2. 复杂类型别名
type Point = (i32, i32);
type Matrix = Vec<Vec<f64>>;

// 3. 函数类型别名
type Calculator = fn(i32, i32) -> i32;

type Callback = Box<dyn FnOnce() -> ()>;

// 4. 泛型类型别名
type GenericResult<T> = Result<T, String>;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    // 1. 使用基本类型别名
    let x: MyInt = 42;
    println!("MyInt value: {}", x);
    
    // 2. 使用复杂类型别名
    let p: Point = (10, 20);
    println!("Point: {:?}", p);
    
    let matrix: Matrix = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0]
    ];
    println!("Matrix: {:?}", matrix);
    
    // 3. 使用函数类型别名
    let calc: Calculator = add;
    println!("5 + 3 = {}", calc(5, 3));
    
    let calc_sub: Calculator = subtract;
    println!("5 - 3 = {}", calc_sub(5, 3));
    
    // 4. 使用泛型类型别名
    let success: GenericResult<i32> = Ok(100);
    let error: GenericResult<i32> = Err("Something went wrong".to_string());
    
    match success {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    match error {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}