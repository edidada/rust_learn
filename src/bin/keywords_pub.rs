#![allow(dead_code)]
// pub 关键字示例

// 1. 声明公共模块
pub mod public_module {
    // 公共函数
    pub fn public_function() {
        println!("Public function called");
    }
    
    // 私有函数
    fn private_function() {
        println!("Private function called");
    }
    
    // 公共结构体
    pub struct PublicStruct {
        pub public_field: i32, // 公共字段
        private_field: i32,    // 私有字段
    }
    
    impl PublicStruct {
        // 公共方法
        pub fn new(public_val: i32, private_val: i32) -> Self {
            Self {
                public_field: public_val,
                private_field: private_val,
            }
        }
        
        // 公共方法
        pub fn get_private_field(&self) -> i32 {
            self.private_field
        }
        
        // 私有方法
        fn private_method(&self) {
            println!("Private method called");
        }
    }
}

// 2. 导入公共模块
use crate::public_module::{public_function, PublicStruct};

fn main() {
    println!("Using pub keyword example");
    
    // 调用公共函数
    public_function();
    
    // 创建公共结构体实例
    let instance = PublicStruct::new(10, 20);
    println!("Public field: {}", instance.public_field);
    println!("Private field (via method): {}", instance.get_private_field());
    
    // 注意：以下操作会导致编译错误
    // instance.private_field; // 无法访问私有字段
    // public_module::private_function(); // 无法访问私有函数
}
