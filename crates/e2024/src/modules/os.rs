// os 模块示例：操作系统特定功能
use std::os;

fn main() {
    // 1. 获取操作系统类型
    println!("1. Operating system type:");
    #[cfg(target_os = "windows")]
    println!("Running on Windows");
    
    #[cfg(target_os = "linux")]
    println!("Running on Linux");
    
    #[cfg(target_os = "macos")]
    println!("Running on macOS");
    
    // 2. 环境变量操作
    println!("\n2. Environment variables:");
    if let Ok(path) = std::env::var("PATH") {
        println!("PATH environment variable exists");
    }
    
    // 3. 临时目录
    println!("\n3. Temporary directory:");
    if let Ok(temp_dir) = std::env::temp_dir() {
        println!("Temp directory: {:?}", temp_dir);
    }
    
    // 4. 命令行参数
    println!("\n4. Command line arguments:");
    let args: Vec<String> = std::env::args().collect();
    println!("Number of arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
    
    // 5. 工作目录
    println!("\n5. Working directory:");
    if let Ok(cwd) = std::env::current_dir() {
        println!("Current working directory: {:?}", cwd);
    }
}