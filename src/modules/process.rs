// process 模块示例：处理进程
use std::process::{Command, Stdio};

fn main() {
    // 1. 执行简单命令
    println!("1. Executing simple command:");
    let output = Command::new("echo")
        .arg("Hello, Rust!")
        .output()
        .expect("Failed to execute command");
    
    println!("Exit status: {:?}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // 2. 执行命令并获取输出
    println!("\n2. Executing command and capturing output:");
    let output = Command::new("cmd")
        .args(["/c", "dir"])
        .output()
        .expect("Failed to execute dir command");
    
    println!("Dir output:\n{}", String::from_utf8_lossy(&output.stdout));
    
    // 3. 执行命令并等待完成
    println!("\n3. Executing command and waiting:");
    let status = Command::new("cmd")
        .args(["/c", "echo", "Command executed"])
        .status()
        .expect("Failed to execute command");
    
    println!("Command exited with status: {:?}", status);
    
    // 4. 设置工作目录
    println!("\n4. Setting working directory:");
    let output = Command::new("cmd")
        .args(["/c", "pwd"])
        .current_dir("./src")
        .output()
        .expect("Failed to execute pwd command");
    
    println!("Current directory:\n{}", String::from_utf8_lossy(&output.stdout));
    
    // 5. 管道命令
    println!("\n5. Piping commands:");
    let output = Command::new("cmd")
        .args(["/c", "echo Hello World | findstr World"])
        .output()
        .expect("Failed to execute piped command");
    
    println!("Piped command output:\n{}", String::from_utf8_lossy(&output.stdout));
}