// LinkedList<T> 示例：双向链表，适用于大量中间插入
use std::collections::LinkedList;

fn main() {
    // 1. 创建LinkedList
    println!("1. Creating LinkedList:");
    let l1: LinkedList<i32> = LinkedList::new();
    let l2 = LinkedList::from([1, 2, 3, 4, 5]);
    
    println!("l1: {:?} (empty: {})", l1, l1.is_empty());
    println!("l2: {:?}", l2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut l = LinkedList::new();
    
    // 从前端添加元素
    l.push_front(1);
    l.push_front(0);
    println!("After push_front: {:?}", l);
    
    // 从后端添加元素
    l.push_back(2);
    l.push_back(3);
    println!("After push_back: {:?}", l);
    
    // 从前端移除元素
    if let Some(first) = l.pop_front() {
        println!("Popped front: {}, list: {:?}", first, l);
    }
    
    // 从后端移除元素
    if let Some(last) = l.pop_back() {
        println!("Popped back: {}, list: {:?}", last, l);
    }
    
    // 长度
    println!("Length: {}", l.len());
    
    // 3. 访问元素
    println!("\n3. Accessing elements:");
    println!("First element: {:?}", l.front());
    println!("Last element: {:?}", l.back());
    
    // 4. 迭代
    println!("\n4. Iteration:");
    println!("Iterating over elements:");
    for &item in &l {
        print!("{} ", item);
    }
    println!();
    
    // 5. 修改元素
    println!("\n5. Modifying elements:");
    if let Some(first) = l.front_mut() {
        *first = 10;
    }
    println!("After modifying first element: {:?}", l);
    
    // 6. 其他方法
    println!("\n6. Other methods:");
    
    // 扩展
    l.extend([4, 5, 6].iter());
    println!("After extend: {:?}", l);
    
    // 分割
    let mut l3 = LinkedList::new();
    l3.push_back(7);
    l3.push_back(8);
    l.append(&mut l3);
    println!("After append: {:?}", l);
    
    // 7. 性能特性
    println!("\n7. Performance characteristics:");
    println!("- 两端插入/删除: O(1)");
    println!("- 中间插入/删除: O(n) 但不需要移动元素");
    println!("- 随机访问: O(n)");
}