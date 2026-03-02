// Vec<T> 示例：动态数组，连续内存，支持随机访问
use std::vec::Vec;

fn main() {
    // 1. 创建Vec
    println!("1. Creating Vec:");
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = Vec::from([10, 20, 30]);
    
    println!("v1: {:?} (empty: {})", v1, v1.is_empty());
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut v = vec![1, 2, 3];
    
    // 添加元素
    v.push(4);
    v.push(5);
    println!("After push: {:?}", v);
    
    // 移除元素
    if let Some(last) = v.pop() {
        println!("Popped: {}, vector: {:?}", last, v);
    }
    
    // 长度和容量
    println!("Length: {}", v.len());
    println!("Capacity: {}", v.capacity());
    
    // 3. 访问元素
    println!("\n3. Accessing elements:");
    println!("First element: {:?}", v.first());
    println!("Last element: {:?}", v.last());
    println!("Element at index 1: {:?}", v.get(1));
    println!("Element at index 10: {:?}", v.get(10));
    
    // 4. 迭代
    println!("\n4. Iteration:");
    println!("Iterating over elements:");
    for &item in &v {
        print!("{} ", item);
    }
    println!();
    
    // 5. 修改元素
    println!("\n5. Modifying elements:");
    v[0] = 10;
    println!("After modifying first element: {:?}", v);
    
    // 6. 其他方法
    println!("\n6. Other methods:");
    
    // 追加另一个Vec
    let mut v4 = vec![6, 7, 8];
    v.append(&mut v4);
    println!("After append: {:?}", v);
    
    // 分割
    let v5 = v.split_off(3);
    println!("After split_off at 3: v={:?}, v5={:?}", v, v5);
    
    // 截断
    v.truncate(2);
    println!("After truncate to 2: {:?}", v);
    
    // 7. 性能特性
    println!("\n7. Performance characteristics:");
    println!("- 随机访问: O(1)");
    println!("- 尾部插入/删除: 均摊O(1)");
    println!("- 中间插入/删除: O(n)");
}