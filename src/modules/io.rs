// io 模块示例：核心I/O功能
use std::io::{self, Read, Write};
use std::vec::Vec;

fn main() -> io::Result<()> {
    // 1. 标准输入输出
    println!("1. Standard input/output:");
    
    // 从标准输入读取
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    println!("Hello, {}!", name.trim());
    
    // 向标准输出写入
    print!("Enter a number: ");
    io::stdout().flush()?;
    
    let mut number = String::new();
    io::stdin().read_line(&mut number)?;
    println!("You entered: {}", number.trim());
    
    // 2. 内存中的I/O
    println!("\n2. In-memory I/O:");
    
    let mut buffer = Vec::new();
    buffer.write_all(b"Hello, Rust!")?;
    
    let mut read_buffer = &buffer[..];
    let mut content = String::new();
    read_buffer.read_to_string(&mut content)?;
    println!("Read from buffer: {}", content);
    
    // 3. 错误处理
    println!("\n3. Error handling:");
    
    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::Other, "Custom error"));
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
