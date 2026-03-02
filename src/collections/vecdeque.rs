// VecDeque<T> 示例：双端队列，两端高效插入删除
use std::collections::VecDeque;

fn main() {
    // 1. 创建VecDeque
    println!("1. Creating VecDeque:");
    let d1: VecDeque<i32> = VecDeque::new();
    let d2 = VecDeque::from([1, 2, 3, 4, 5]);
    
    println!("d1: {:?} (empty: {})", d1, d1.is_empty());
    println!("d2: {:?}", d2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut d = VecDeque::new();
    
    // 从前端添加元素
    d.push_front(1);
    d.push_front(0);
    println!("After push_front: {:?}", d);
    
    // 从后端添加元素
    d.push_back(2);
    d.push_back(3);
    println!("After push_back: {:?}", d);
    
    // 从前端移除元素
    if let Some(first) = d.pop_front() {
        println!("Popped front: {}, deque: {:?}", first, d);
    }
    
    // 从后端移除元素
    if let Some(last) = d.pop_back() {
        println!("Popped back: {}, deque: {:?}", last, d);
    }
    
    // 长度
    println!("Length: {}", d.len());
    
    // 3. 访问元素
    println!("\n3. Accessing elements:");
    println!("First element: {:?}", d.front());
    println!("Last element: {:?}", d.back());
    println!("Element at index 1: {:?}", d.get(1));
    
    // 4. 迭代
    println!("\n4. Iteration:");
    println!("Iterating over elements:");
    for &item in &d {
        print!("{} ", item);
    }
    println!();
    
    // 5. 修改元素
    println!("\n5. Modifying elements:");
    if let Some(first) = d.front_mut() {
        *first = 10;
    }
    println!("After modifying first element: {:?}", d);
    
    // 6. 其他方法
    println!("\n6. Other methods:");
    
    // 扩展
    d.extend([4, 5, 6].iter());
    println!("After extend: {:?}", d);
    
    // 旋转
    d.rotate_left(2);
    println!("After rotate_left(2): {:?}", d);
    
    // 7. 性能特性
    println!("\n7. Performance characteristics:");
    println!("- 两端插入/删除: O(1)");
    println!("- 随机访问: O(1)");
    println!("- 中间插入/删除: O(n)");
}