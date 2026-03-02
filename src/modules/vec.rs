// vec 模块示例：具有堆分配内容的连续可增长数组类型
use std::vec::Vec;

fn main() {
    // 1. 创建向量
    println!("1. Creating vectors:");
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = Vec::from([10, 20, 30]);
    
    println!("v1: {:?} (empty: {})", v1, v1.is_empty());
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
    
    // 2. 向量操作
    println!("\n2. Vector operations:");
    let mut v = vec![1, 2, 3];
    
    // push
    v.push(4);
    v.push(5);
    println!("After push: {:?}", v);
    
    // pop
    if let Some(last) = v.pop() {
        println!("Popped: {}, vector: {:?}", last, v);
    }
    
    // len
    println!("Length: {}", v.len());
    
    // capacity
    println!("Capacity: {}", v.capacity());
    
    // reserve
    v.reserve(10);
    println!("Capacity after reserve: {}", v.capacity());
    
    // 3. 访问元素
    println!("\n3. Accessing elements:");
    println!("First element: {:?}", v.first());
    println!("Last element: {:?}", v.last());
    println!("Element at index 1: {:?}", v.get(1));
    println!("Element at index 10: {:?}", v.get(10));
    
    // 4. 向量迭代
    println!("\n4. Vector iteration:");
    println!("Iterating over elements:");
    for &item in &v {
        print!("{} ", item);
    }
    println!();
    
    // 5. 向量修改
    println!("\n5. Modifying vector:");
    v[0] = 10;
    println!("After modifying first element: {:?}", v);
    
    // 6. 向量方法
    println!("\n6. Vector methods:");
    
    // append
    let mut v4 = vec![6, 7, 8];
    v.append(&mut v4);
    println!("After append: {:?}", v);
    
    // split_off
    let v5 = v.split_off(3);
    println!("After split_off at 3: v={:?}, v5={:?}", v, v5);
    
    // truncate
    v.truncate(2);
    println!("After truncate to 2: {:?}", v);
    
    // 7. 向量转换
    println!("\n7. Vector conversions:");
    let v6 = vec![1, 2, 3];
    let slice: &[i32] = &v6;
    println!("Vector as slice: {:?}", slice);
    
    let array: [i32; 3] = v6.try_into().unwrap();
    println!("Vector to array: {:?}", array);
}