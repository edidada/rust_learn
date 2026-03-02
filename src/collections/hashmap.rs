// HashMap<K, V> 示例：哈希表实现，无序存储
use std::collections::HashMap;

fn main() {
    // 1. 创建HashMap
    println!("1. Creating HashMap:");
    let mut map1: HashMap<String, i32> = HashMap::new();
    
    let mut map2 = HashMap::new();
    map2.insert("Alice", 30);
    map2.insert("Bob", 25);
    map2.insert("Charlie", 35);
    
    println!("map1: {:?} (empty: {})", map1, map1.is_empty());
    println!("map2: {:?}", map2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut map = HashMap::new();
    
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
    
    // 4. 其他方法
    println!("\n4. Other methods:");
    
    // 检查键是否存在
    println!("Contains key 'Banana': {}", map.contains_key("Banana"));
    
    // 插入或更新
    let old_value = map.insert("Banana", 25);
    println!("Old value of Banana: {:?}", old_value);
    println!("After update: {:?}", map);
    
    // 获取或插入
    let value = map.entry("Grape").or_insert(30);
    println!("Value of Grape: {}", value);
    println!("After or_insert: {:?}", map);
    
    // 5. 性能特性
    println!("\n5. Performance characteristics:");
    println!("- 插入: 均摊O(1)");
    println!("- 查找: 均摊O(1)");
    println!("- 删除: 均摊O(1)");
    println!("- 遍历: O(n)");
    println!("- 顺序: 无序存储");
}