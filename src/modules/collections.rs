// collections 模块示例：集合类型
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    // Vec：可变大小的数组
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    println!("Vec: {:?}", vec);
    
    // HashMap：键值对映射
    let mut map = HashMap::new();
    map.insert("Alice", 30);
    map.insert("Bob", 25);
    map.insert("Charlie", 35);
    println!("HashMap: {:?}", map);
    
    // 查找值
    if let Some(age) = map.get("Alice") {
        println!("Alice's age: {}", age);
    }
    
    // HashSet：无序集合
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(1); // 重复元素会被忽略
    println!("HashSet: {:?}", set);
    println!("Contains 2? {}", set.contains(&2));
    
    // LinkedList：双向链表
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    println!("LinkedList: {:?}", list);
    
    // VecDeque：双端队列
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.pop_back();
    println!("VecDeque: {:?}", deque);
}