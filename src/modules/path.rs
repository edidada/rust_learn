// path 模块示例：跨平台路径操作
use std::path::{Path, PathBuf};

fn main() {
    // 1. 创建路径
    println!("1. Creating paths:");
    let path1 = Path::new("./src");
    let path2 = Path::new("C:\\Users\\example\\Desktop"); // Windows路径
    let path3 = Path::new("/home/example"); // Unix路径
    
    println!("Path 1: {:?}", path1);
    println!("Path 2: {:?}", path2);
    println!("Path 3: {:?}", path3);
    
    // 2. 路径操作
    println!("\n2. Path operations:");
    let mut path_buf = PathBuf::from("./src");
    path_buf.push("modules");
    path_buf.push("path.rs");
    println!("Constructed path: {:?}", path_buf);
    
    // 3. 路径组件
    println!("\n3. Path components:");
    for component in path_buf.components() {
        println!("Component: {:?}", component);
    }
    
    // 4. 路径属性
    println!("\n4. Path properties:");
    println!("Is absolute: {}", path_buf.is_absolute());
    println!("Is relative: {}", path_buf.is_relative());
    println!("Has root: {}", path_buf.has_root());
    
    // 5. 路径信息
    println!("\n5. Path information:");
    if let Some(file_name) = path_buf.file_name() {
        println!("File name: {:?}", file_name);
    }
    
    if let Some(parent) = path_buf.parent() {
        println!("Parent: {:?}", parent);
    }
    
    if let Some(stem) = path_buf.file_stem() {
        println!("File stem: {:?}", stem);
    }
    
    if let Some(extension) = path_buf.extension() {
        println!("File extension: {:?}", extension);
    }
    
    // 6. 路径转换
    println!("\n6. Path conversion:");
    if let Some(path_str) = path_buf.to_str() {
        println!("Path as string: {}", path_str);
    }
    
    // 7. 路径连接
    println!("\n7. Path joining:");
    let base_path = Path::new("./src");
    let joined_path = base_path.join("modules").join("path.rs");
    println!("Joined path: {:?}", joined_path);
}