#![allow(unused_imports)]
// use 关键字示例

// 1. 基本导入
use std::fmt::Display;
use std::collections::HashMap;

// 2. 重命名导入
use std::collections::HashSet as Set;
use std::io::Result as IoResult;

// 3. 导入整个模块
use std::time;

// 4. 导入模块的所有内容
use std::fs::*;

// 5. 嵌套导入
use std::sync::{Arc, Mutex};

// 6. 从当前crate导入
mod utils {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    }
}

use crate::utils::math::{add, subtract};

fn main() {
    println!("Using use keyword example");
    
    // 1. 使用导入的类型
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("HashMap: {:?}", map);
    
    // 2. 使用重命名的类型
    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    println!("HashSet: {:?}", set);
    
    // 3. 使用导入的模块
    let now = time::Instant::now();
    println!("Current time: {:?}", now);
    
    // 4. 使用通配符导入
    // let file = File::open("test.txt"); // 取消注释以测试
    
    // 5. 使用嵌套导入
    let shared_value = Arc::new(Mutex::new(42));
    println!("Shared value: {:?}", shared_value);
    
    // 6. 使用从当前crate导入的函数
    let sum = add(5, 7);
    let difference = subtract(10, 3);
    println!("5 + 7 = {}", sum);
    println!("10 - 3 = {}", difference);
}