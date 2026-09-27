// slice 模块示例：slice原始类型的工具
use std::slice;

fn main() {
    // 1. 基本slice操作
    println!("1. Basic slice operations:");
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    
    println!("Original array: {:?}", arr);
    println!("Slice [1..4]: {:?}", slice);
    println!("Slice length: {}", slice.len());
    println!("Slice is empty: {}", slice.is_empty());
    
    // 2. 切片方法
    println!("\n2. Slice methods:");
    
    // first 和 last
    println!("First element: {:?}", slice.first());
    println!("Last element: {:?}", slice.last());
    
    // get
    println!("Element at index 1: {:?}", slice.get(1));
    println!("Element at index 10: {:?}", slice.get(10));
    
    // split_first 和 split_last
    if let Some((first, rest)) = slice.split_first() {
        println!("Split first: {}, rest: {:?}", first, rest);
    }
    
    // 3. 切片转换
    println!("\n3. Slice conversions:");
    let mut mutable_arr = [1, 2, 3, 4, 5];
    let mutable_slice = &mut mutable_arr[..];
    
    // 修改切片
    mutable_slice[0] = 10;
    println!("After modification: {:?}", mutable_arr);
    
    // 4. 切片迭代
    println!("\n4. Slice iteration:");
    println!("Iterating over slice:");
    for &item in slice {
        print!("{} ", item);
    }
    println!();
    
    // 5. 切片搜索
    println!("\n5. Slice searching:");
    let numbers = [1, 2, 3, 4, 5, 4, 3, 2, 1];
    
    if let Some(index) = numbers.iter().position(|&x| x == 5) {
        println!("Found 5 at index: {}", index);
    }
    
    // 6. 切片排序
    println!("\n6. Slice sorting:");
    let mut unsorted = [5, 3, 1, 4, 2];
    println!("Unsorted: {:?}", unsorted);
    
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
}