// mod 关键字示例

// 1. 声明模块
mod utils {
    // 模块内的函数
    pub fn helper() {
        println!("Helper function called");
    }
    
    // 模块内的子模块
    mod inner {
        pub fn inner_helper() {
            println!("Inner helper function called");
        }
    }
}

// 2. 导入模块内容
use utils::helper;
use utils::inner::inner_helper;

fn main() {
    println!("Using mod keyword example");
    
    // 调用模块函数
    helper();
    inner_helper();
    
    // 直接使用模块路径
    utils::helper();
    utils::inner::inner_helper();
}