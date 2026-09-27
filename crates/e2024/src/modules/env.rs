// env 模块示例：检查和操作进程环境
use std::env;

fn main() {
    // 1. 获取环境变量
    println!("1. Getting environment variables:");
    if let Ok(path) = env::var("PATH") {
        println!("PATH environment variable exists");
        // 打印前100个字符
        println!("PATH starts with: {}", &path[..std::cmp::min(100, path.len())]);
    }
    
    // 2. 获取不存在的环境变量
    match env::var("NON_EXISTENT_VAR") {
        Ok(value) => println!("NON_EXISTENT_VAR: {}", value),
        Err(e) => println!("NON_EXISTENT_VAR error: {}", e),
    }
    
    // 3. 获取命令行参数
    println!("\n2. Command line arguments:");
    let args: Vec<String> = env::args().collect();
    println!("Number of arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
    
    // 4. 获取当前工作目录
    println!("\n3. Current working directory:");
    if let Ok(cwd) = env::current_dir() {
        println!("CWD: {:?}", cwd);
    }
    
    // 5. 获取可执行文件路径
    println!("\n4. Executable path:");
    if let Ok(exe_path) = env::current_exe() {
        println!("Executable path: {:?}", exe_path);
    }
    
    // 6. 获取系统临时目录
    println!("\n5. Temp directory:");
    if let Ok(temp_dir) = env::temp_dir() {
        println!("Temp directory: {:?}", temp_dir);
    }
}