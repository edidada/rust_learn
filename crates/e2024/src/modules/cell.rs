// cell 模块示例：可共享的可变容器
use std::cell::{Cell, RefCell};

fn main() {
    // Cell 示例（适用于Copy类型）
    let cell = Cell::new(42);
    println!("Initial value: {}", cell.get());
    
    cell.set(100);
    println!("Updated value: {}", cell.get());
    
    // RefCell 示例（适用于非Copy类型）
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    
    // 不可变借用
    { 
        let borrowed = ref_cell.borrow();
        println!("Borrowed value: {:?}", borrowed);
    } // 借用结束
    
    // 可变借用
    { 
        let mut borrowed_mut = ref_cell.borrow_mut();
        borrowed_mut.push(4);
        println!("Modified value: {:?}", borrowed_mut);
    } // 借用结束
    
    // 再次不可变借用
    let borrowed = ref_cell.borrow();
    println!("Final value: {:?}", borrowed);
}