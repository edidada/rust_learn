// super 关键字示例

// 父模块
mod parent {
    pub fn parent_function() {
        println!("Parent function called");
    }
    
    // 子模块
    pub mod child {
        pub fn child_function() {
            println!("Child function called");
            // 使用super引用父模块
            super::parent_function();
        }
        
        // 孙子模块
        pub mod grandchild {
            pub fn grandchild_function() {
                println!("Grandchild function called");
                // 使用super引用父模块（child）
                super::child_function();
                // 使用super::super引用祖父模块（parent）
                super::super::parent_function();
            }
        }
    }
}

fn main() {
    println!("Using super keyword example");
    
    // 调用子模块函数
    parent::child::child_function();
    println!();
    
    // 调用孙子模块函数
    parent::child::grandchild::grandchild_function();
}