// HashSet<T> 示例：基于 HashMap 的集合实现
use std::collections::HashSet;

fn main() {
    // 1. 创建HashSet
    println!("1. Creating HashSet:");
    let set1: HashSet<i32> = HashSet::new();
    let set2 = HashSet::from([1, 2, 3, 4, 5]);
    
    println!("set1: {:?} (empty: {})", set1, set1.is_empty());
    println!("set2: {:?}", set2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut set = HashSet::new();
    
    // 添加元素
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("After insert: {:?}", set);
    
    // 插入重复元素
    let inserted = set.insert(2);
    println!("Inserted 2 again: {}", inserted);
    println!("Set after inserting duplicate: {:?}", set);
    
    // 移除元素
    let removed = set.remove(&1);
    println!("Removed 1: {}", removed);
    println!("After remove: {:?}", set);
    
    // 长度
    println!("Length: {}", set.len());
    
    // 3. 检查元素
    println!("\n3. Checking elements:");
    println!("Contains 2: {}", set.contains(&2));
    println!("Contains 10: {}", set.contains(&10));
    
    // 4. 迭代
    println!("\n4. Iteration:");
    println!("Iterating over elements:");
    for &item in &set {
        print!("{} ", item);
    }
    println!();
    
    // 5. 集合操作
    println!("\n5. Set operations:");
    let set_a = HashSet::from([1, 2, 3, 4]);
    let set_b = HashSet::from([3, 4, 5, 6]);
    
    // 并集
    let union: HashSet<_> = set_a.union(&set_b).collect();
    println!("Union: {:?}", union);
    
    // 交集
    let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
    println!("Intersection: {:?}", intersection);
    
    // 差集
    let difference: HashSet<_> = set_a.difference(&set_b).collect();
    println!("Difference: {:?}", difference);
    
    // 对称差
    let symmetric_difference: HashSet<_> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric difference: {:?}", symmetric_difference);
    
    // 6. 性能特性
    println!("\n6. Performance characteristics:");
    println!("- 插入: 均摊O(1)");
    println!("- 查找: 均摊O(1)");
    println!("- 删除: 均摊O(1)");
    println!("- 遍历: O(n)");
    println!("- 顺序: 无序存储");
}