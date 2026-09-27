// alloc 模块示例：内存分配API
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    // 分配内存
    let layout = Layout::new::<i32>();
    let ptr = unsafe {
        alloc(layout)
    };
    
    // 写入数据
    unsafe {
        *ptr.cast::<i32>() = 42;
        println!("Allocated value: {}", *ptr.cast::<i32>());
    }
    
    // 释放内存
    unsafe {
        dealloc(ptr, layout);
    }
    
    println!("Memory allocated and freed successfully");
}