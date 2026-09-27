// marker 模块示例：表示类型基本属性的原始特性和类型
use std::marker::{Send, Sync, PhantomData};

// 1. Send 和 Sync trait
// Send: 类型可以安全地从一个线程发送到另一个线程
// Sync: 类型可以安全地被多个线程同时访问

// 2. PhantomData
struct MyStruct<T> {
    // PhantomData 表示我们逻辑上拥有 T 类型的值
    // 但实际上不存储任何 T 类型的数据
    phantom: PhantomData<T>,
    value: u32,
}

impl<T> MyStruct<T> {
    fn new(value: u32) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }
}

// 3. 条件实现trait
struct SendableStruct;
struct NonSendableStruct(*mut u32);

// SendableStruct 实现 Send
unsafe impl Send for SendableStruct {}

// NonSendableStruct 不实现 Send
// unsafe impl Send for NonSendableStruct {} // 取消注释会导致未定义行为

fn main() {
    // 1. 使用 PhantomData
    let my_struct: MyStruct<String> = MyStruct::new(42);
    println!("MyStruct value: {}", my_struct.value);
    
    // 2. 演示 Send trait
    println!("\nSend trait demonstration:");
    println!("SendableStruct implements Send: {}", std::marker::Send::is_implied_for::<SendableStruct>());
    println!("NonSendableStruct implements Send: {}", std::marker::Send::is_implied_for::<NonSendableStruct>());
    
    // 3. 演示 Sync trait
    println!("\nSync trait demonstration:");
    println!("SendableStruct implements Sync: {}", std::marker::Sync::is_implied_for::<SendableStruct>());
    println!("NonSendableStruct implements Sync: {}", std::marker::Sync::is_implied_for::<NonSendableStruct>());
    
    // 4. 使用 PhantomData 进行生命周期管理
    struct Borrowed<'a, T> {
        data: &'a T,
        _marker: PhantomData<&'a T>,
    }
    
    let value = 42;
    let borrowed = Borrowed {
        data: &value,
        _marker: PhantomData,
    };
    
    println!("\nBorrowed value: {}", borrowed.data);
}