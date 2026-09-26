// as 关键字示例

// 1. 类型转换
fn type_conversion() {
    let x: i32 = 42;
    let y: i64 = x as i64;
    println!("i32 to i64: {} -> {}", x, y);
    
    let c: char = 'A';
    let ascii: u8 = c as u8;
    println!("char to u8: {} -> {}", c, ascii);
}

// 2. 重命名导入
use std::collections::HashMap as Map;
use std::fmt::Display as FmtDisplay;

// 3. 限定路径到关联项
struct MyStruct;

impl MyStruct {
    const VALUE: i32 = 100;
    fn method(&self) {}
}

fn main() {
    // 1. 类型转换示例
    type_conversion();
    
    // 2. 重命名导入示例
    let mut map = Map::new();
    map.insert("key", "value");
    println!("Map: {:?}", map);
    
    // 3. 限定路径到关联项示例
    println!("MyStruct::VALUE: {}", MyStruct::VALUE);
    let instance = MyStruct;
    instance.method();
    println!("Called MyStruct::method()");
}