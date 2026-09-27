// error 模块示例：处理错误的接口
use std::error::Error;
use std::fmt;

// 1. 自定义错误类型
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

// 2. 函数返回Result
fn divide(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
    if b == 0 {
        return Err(Box::new(MyError { message: "Division by zero".to_string() }));
    }
    Ok(a / b)
}

// 3. 错误链
#[derive(Debug)]
struct WrapperError {
    source: MyError,
}

impl fmt::Display for WrapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrapperError: {}", self.source)
    }
}

impl Error for WrapperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn wrapped_divide(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
    divide(a, b).map_err(|e| {
        Box::new(WrapperError { source: MyError { message: e.to_string() } })
    })
}

fn main() {
    // 1. 处理错误
    println!("1. Handling errors:");
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 2. 处理错误链
    println!("\n2. Error chaining:");
    match wrapped_divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("Source error: {}", source);
            }
        },
    }
    
    // 3. 使用 ? 操作符
    println!("\n3. Using ? operator:");
    fn example() -> Result<(), Box<dyn Error>> {
        let result = divide(20, 4)?;
        println!("20 / 4 = {}", result);
        Ok(())
    }
    
    if let Err(e) = example() {
        println!("Example error: {}", e);
    }
}