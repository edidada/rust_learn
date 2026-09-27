// fs 模块示例：文件系统操作
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    // 1. 创建目录
    println!("1. Creating directory:");
    fs::create_dir_all("test_dir")?;
    println!("Created directory 'test_dir'");
    
    // 2. 写入文件
    println!("\n2. Writing to file:");
    fs::write("test_dir/test.txt", "Hello, Rust!")?;
    println!("Wrote to 'test_dir/test.txt'");
    
    // 3. 读取文件
    println!("\n3. Reading from file:");
    let content = fs::read_to_string("test_dir/test.txt")?;
    println!("File content: {}", content);
    
    // 4. 列出目录内容
    println!("\n4. Listing directory contents:");
    let entries = fs::read_dir("test_dir")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("{:?}", path);
    }
    
    // 5. 检查文件是否存在
    println!("\n5. Checking if file exists:");
    let exists = fs::metadata("test_dir/test.txt").is_ok();
    println!("test.txt exists: {}", exists);
    
    // 6. 删除文件
    println!("\n6. Deleting file:");
    fs::remove_file("test_dir/test.txt")?;
    println!("Deleted 'test_dir/test.txt'");
    
    // 7. 删除目录
    println!("\n7. Deleting directory:");
    fs::remove_dir("test_dir")?;
    println!("Deleted 'test_dir'");
    
    Ok(())
}