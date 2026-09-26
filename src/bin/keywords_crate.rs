// crate 关键字示例

// 1. 引用当前 crate 的根模块
mod utils {
    pub fn helper() {
        println!("Helper function called");
    }
}

// 2. 使用 crate 关键字引用模块
use crate::utils::helper;

// 3. 在库中，crate 表示库本身
fn main() {
    println!("Using crate keyword example");
    
    // 调用通过 crate 路径引用的函数
    helper();
    
    // 直接使用 crate 作为根模块路径
    crate::utils::helper();
}