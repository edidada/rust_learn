// self 关键字示例

// 1. 作为方法接收者
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self - 不可变引用
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // &mut self - 可变引用
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // self - 所有权
    fn into_square(self) -> Square {
        let side = std::cmp::min(self.width, self.height);
        Square { side }
    }
}

struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

// 2. 作为当前模块
mod utils {
    pub fn helper() {
        println!("Helper function");
    }
    
    pub mod inner {
        pub fn inner_helper() {
            // 使用self引用当前模块
            println!("Inner helper function");
            // 引用同级模块
            super::helper();
        }
    }
}

fn main() {
    // 1. 使用self作为方法接收者
    let mut rect = Rectangle { width: 10, height: 20 };
    println!("Initial area: {}", rect.area());
    
    rect.double_size();
    println!("After doubling: {}", rect.area());
    
    let square = rect.into_square();
    println!("Square area: {}", square.area());
    
    // 2. 使用self作为当前模块
    utils::inner::inner_helper();
}