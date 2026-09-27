// pin 模块示例：将数据固定到内存位置的类型
use std::pin::Pin;
use std::marker::PhantomPinned;

// 1. 固定的数据结构
struct PinnedStruct {
    data: i32,
    // PhantomPinned 防止结构被移动
    _pin: PhantomPinned,
}

impl PinnedStruct {
    fn new(data: i32) -> Self {
        Self {
            data,
            _pin: PhantomPinned,
        }
    }
    
    // 需要固定才能调用的方法
    fn set_data(self: Pin<&mut Self>, new_data: i32) {
        // 安全地获取可变引用
        unsafe {
            let this = self.get_unchecked_mut();
            this.data = new_data;
        }
    }
    
    fn get_data(self: Pin<&Self>) -> i32 {
        self.data
    }
}

fn main() {
    // 1. 使用Pin
    println!("1. Using Pin:");
    
    // 创建一个PinnedStruct
    let mut unpinned = PinnedStruct::new(42);
    
    // 固定它
    let mut pinned = unsafe {
        Pin::new_unchecked(&mut unpinned)
    };
    
    println!("Initial data: {}", pinned.get_data());
    
    // 修改数据
    pinned.set_data(100);
    println!("Updated data: {}", pinned.get_data());
    
    // 2. Pin与Box
    println!("\n2. Pin with Box:");
    let boxed = Box::pin(PinnedStruct::new(200));
    println!("Boxed pinned data: {}", boxed.get_data());
    
    // 3. Pin与async/await
    println!("\n3. Pin with async/await:");
    // async函数返回的Future需要被固定
    async fn async_function() -> i32 {
        42
    }
    
    let future = async_function();
    // 当使用.await时，Future会被自动固定
    // let result = future.await;
    // println!("Async result: {}", result);
    
    println!("Pin module examples");
}