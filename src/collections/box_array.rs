// Box<[T]> 示例：固定大小数组，运行时决定长度

fn main() {
    // 1. 创建Box<[T]>
    println!("1. Creating Box<[T]:");
    
    // 从Vec转换
    let vec = vec![1, 2, 3, 4, 5];
    let boxed_slice: Box<[i32]> = vec.into_boxed_slice();
    println!("From Vec: {:?}", boxed_slice);
    
    // 使用Box::new创建
    let boxed_array: Box<[i32; 3]> = Box::new([10, 20, 30]);
    println!("From array: {:?}", boxed_array);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let boxed = vec![1, 2, 3, 4, 5].into_boxed_slice();
    
    // 长度
    println!("Length: {}", boxed.len());
    
    // 是否为空
    println!("Is empty: {}", boxed.is_empty());
    
    // 3. 访问元素
    println!("\n3. Accessing elements:");
    println!("First element: {:?}", boxed.first());
    println!("Last element: {:?}", boxed.last());
    println!("Element at index 2: {:?}", boxed.get(2));
    
    // 4. 迭代
    println!("\n4. Iteration:");
    println!("Iterating over elements:");
    for &item in &boxed {
        print!("{} ", item);
    }
    println!();
    
    // 5. 修改元素
    println!("\n5. Modifying elements:");
    let mut mut_boxed = vec![1, 2, 3].into_boxed_slice();
    if let Some(elem) = mut_boxed.get_mut(0) {
        *elem = 10;
    }
    println!("After modification: {:?}", mut_boxed);
    
    // 6. 转换回Vec
    println!("\n6. Converting back to Vec:");
    let vec_from_boxed = mut_boxed.into_vec();
    println!("Vec from Box<[T]>: {:?}", vec_from_boxed);
    
    // 7. 性能特性
    println!("\n7. Performance characteristics:");
    println!("- 随机访问: O(1)");
    println!("- 内存布局: 连续内存");
    println!("- 大小: 运行时决定，固定大小");
}