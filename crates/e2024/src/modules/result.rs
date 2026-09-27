// result 模块示例：错误处理与Result类型
use std::result::Result;
use std::error::Error;
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// 可能失败的函数
fn divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "Division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

fn main() {
    // 1. 基本Result使用
    println!("1. Basic Result usage:");
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 2. 使用?操作符
    println!("\n2. Using ? operator:");
    fn calculate(a: i32, b: i32) -> Result<i32, MyError> {
        let result = divide(a, b)?;
        Ok(result * 2)
    }
    
    match calculate(20, 4) {
        Ok(result) => println!("(20 / 4) * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 3. 链式方法
    println!("\n3. Chaining methods:");
    let result = divide(15, 3)
        .map(|x| x * 2)
        .map_err(|e| MyError { message: format!("Calculation error: {}", e) });
    
    match result {
        Ok(value) => println!("Chained result: {}", value),
        Err(e) => println!("Chained error: {}", e),
    }
    
    // 4. 转换为Option
    println!("\n4. Converting to Option:");
    let option_result = divide(10, 2).ok();
    println!("Ok result as Option: {:?}", option_result);
    
    let option_error = divide(10, 0).ok();
    println!("Error result as Option: {:?}", option_error);
    
    // 5. 组合Result
    println!("\n5. Combining Result:");
    fn multiply(a: Result<i32, MyError>, b: Result<i32, MyError>) -> Result<i32, MyError> {
        a.and_then(|x| b.map(|y| x * y))
    }
    
    let result1 = multiply(Ok(2), Ok(3));
    println!("multiply(Ok(2), Ok(3)): {:?}", result1);
    
    let result2 = multiply(Ok(2), Err(MyError { message: "Error".to_string() }));
    println!("multiply(Ok(2), Err(...)): {:?}", result2);
}