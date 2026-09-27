// BTreeSet<T> 示例：基于 BTreeMap 的有序集合
use std::collections::BTreeSet;

fn main() {
    // 1. 创建BTreeSet
    println!("1. Creating BTreeSet:");
    let set1: BTreeSet<i32> = BTreeSet::new();
    let set2 = BTreeSet::from([3, 1, 4, 1, 5, 9, 2, 6]);
    
    println!("set1: {:?} (empty: {})", set1, set1.is_empty());
    println!("set2: {:?}", set2);
    
    // 2. 基本操作
    println!("\n2. Basic operations:");
    let mut set = BTreeSet::new();
    
    // 添加元素
    set.insert(3);
    set.insert(1);
    set.insert(4);
    set.insert(1); // 重复元素
    set.insert(2);
    println!("After insert: {:?}", set);
    
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
    
    // 5. 范围操作
    println!("\n5. Range operations:");
    let set3 = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    
    println!("Range 3..7:");
    for &item in set3.range(3..7) {
        print!("{} ", item);
    }
    println!();
    
    // 6. 集合操作
    println!("\n6. Set operations:");
    let set_a = BTreeSet::from([1, 2, 3, 4]);
    let set_b = BTreeSet::from([3, 4, 5, 6]);
    
    // 并集
    let union: BTreeSet<_> = set_a.union(&set_b).collect();
    println!("Union: {:?}", union);
    
    // 交集
    let intersection: BTreeSet<_> = set_a.intersection(&set_b).collect();
    println!("Intersection: {:?}", intersection);
    
    // 差集
    let difference: BTreeSet<_> = set_a.difference(&set_b).collect();
    println!("Difference: {:?}", difference);
    
    // 对称差
    let symmetric_difference: BTreeSet<_> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric difference: {:?}", symmetric_difference);
    
    // 7. 性能特性
    println!("\n7. Performance characteristics:");
    println!("- 插入: O(log n)");
    println!("- 查找: O(log n)");
    println!("- 删除: O(log n)");
    println!("- 遍历: O(n)");
    println!("- 顺序: 自动维护有序性");
}