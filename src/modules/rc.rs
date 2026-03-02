// rc 模块示例：单线程引用计数指针
use std::rc::Rc;

fn main() {
    // 1. 创建Rc
    println!("1. Creating Rc:");
    let rc1 = Rc::new(42);
    println!("rc1 value: {}", rc1);
    println!("rc1 strong count: {}", Rc::strong_count(&rc1));
    
    // 2. 克隆Rc
    println!("\n2. Cloning Rc:");
    let rc2 = Rc::clone(&rc1);
    let rc3 = rc1.clone();
    
    println!("rc2 value: {}", rc2);
    println!("rc3 value: {}", rc3);
    println!("Strong count after cloning: {}", Rc::strong_count(&rc1));
    
    // 3. 共享可变数据（需要配合RefCell）
    println!("\n3. Shared mutable data:");
    use std::cell::RefCell;
    
    let shared_data = Rc::new(RefCell::new(0));
    println!("Initial value: {:?}", shared_data.borrow());
    
    let data_clone1 = Rc::clone(&shared_data);
    let data_clone2 = Rc::clone(&shared_data);
    
    // 修改数据
    *data_clone1.borrow_mut() = 10;
    println!("After modification through data_clone1: {:?}", shared_data.borrow());
    
    *data_clone2.borrow_mut() = 20;
    println!("After modification through data_clone2: {:?}", shared_data.borrow());
    
    // 4. 当Rc离开作用域时，引用计数减少
    println!("\n4. Reference count decrease:");
    {
        let rc4 = Rc::clone(&rc1);
        println!("Strong count inside scope: {}", Rc::strong_count(&rc1));
    }
    println!("Strong count outside scope: {}", Rc::strong_count(&rc1));
    
    // 5. 弱引用
    println!("\n5. Weak references:");
    use std::rc::Weak;
    
    let rc5 = Rc::new("Hello".to_string());
    let weak = Rc::downgrade(&rc5);
    
    println!("Weak reference upgraded: {:?}", weak.upgrade());
    
    // 当Rc被丢弃时，弱引用会返回None
    drop(rc5);
    println!("Weak reference upgraded after dropping Rc: {:?}", weak.upgrade());
}