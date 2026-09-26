// BTreeMap<K, V> 示例：基于 B-树 的有序映射，按键排序
use std::collections::BTreeMap;

fn main() {
    // 1. 创建BTreeMap
    println!("1. Creating BTreeMap:");
    let mut map1: BTreeMap<String, i32> = BTreeMap::new();
    
    let mut map2 = BTreeMap::new();
    map2.insert("Alice", 30);
    map2.insert("Bob", 25);
    map2.insert("Charlie", 35);
    
    println!("map1: {:?} (empty: {})", map1, map1.is_empty());
    println!("map2: {:?}", map2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut map = BTreeMap::new();
    
    // 插入键值对
    map.insert("Apple", 10);
    map.insert("Banana", 20);
    map.insert("Orange", 15);
    println!("After insert: {:?}", map);
    
    // 查找值
    if let Some(&value) = map.get("Banana") {
        println!("Banana: {}", value);
    }
    
    // 移除键值对
    if let Some(value) = map.remove("Apple") {
        println!("Removed Apple: {}", value);
    }
    println!("After remove: {:?}", map);
    
    // 长度
    println!("Length: {}", map.len());
    
    // 3. 迭代
    println!("\n3. Iteration:");
    println!("Iterating over key-value pairs:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // 4. 范围操作
    println!("\n4. Range operations:");
    let mut map3 = BTreeMap::new();
    map3.insert(1, "One");
    map3.insert(2, "Two");
    map3.insert(3, "Three");
    map3.insert(4, "Four");
    map3.insert(5, "Five");
    
    println!("Range 2..4:");
    for (key, value) in map3.range(2..4) {
        println!("{}: {}", key, value);
    }
    
    // 5. 其他方法
    println!("\n5. Other methods:");
    
    // 检查键是否存在
    println!("Contains key 'Banana': {}", map.contains_key("Banana"));
    
    // 获取第一个和最后一个元素
    println!("First entry: {:?}", map.first_entry());
    println!("Last entry: {:?}", map.last_entry());
    
    // 6. 性能特性
    println!("\n6. Performance characteristics:");
    println!("- 插入: O(log n)");
    println!("- 查找: O(log n)");
    println!("- 删除: O(log n)");
    println!("- 遍历: O(n)");
    println!("- 按键排序: 自动维护有序性");
}