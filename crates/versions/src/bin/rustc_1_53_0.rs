// rustc 1.53.0 演示 —— or-patterns、IntoIterator for [T;N]、u32::BITS
fn main() {
    println!("rustc 1.53.0 演示");

    // ============================================================
    println!("\n1. or-patterns：模式内部可用 | ");
    // 1.53 起 | 可嵌套子模式。对比旧行为：只能整段连，需要重复外层模式。
    let x = Some(2u8);
    // 旧写法：matches!(x, Some(1) | Some(2))
    let ok = matches!(x, Some(1 | 2 | 3));
    println!("  matches!(Some(2u8), Some(1 | 2 | 3)) = {}", ok);
    match Some(4u8) {
        Some(1 | 2) => println!("  小值"),
        Some(n) => println!("  其它值: {}", n),
        None => println!("  无值"),
    }

    // ============================================================
    println!("\n2. IntoIterator for [T; N] —— 数组逐元素迭代（1.53 重点）");
    // 对比旧行为：1.53 前 arr.into_iter() 在方法/for 循环里只会借出 &T
    //             （新代码会警告：数组旁的 .into_iter() 语义模糊）。
    // 直接 IntoIterator::into_iter(arr) 则按值移动。
    let sizes = [10u32, 20, 30];
    let mut total = 0;
    for v in sizes {
        // for 循环里 v 是 u32（按值），1.53 化
        total += v;
    }
    println!("  for v in [10,20,30] 累计 = {}", total);
    // 通用泛型代码终于能直接收数组：
    fn sum<I: IntoIterator<Item = u32>>(iter: I) -> u32 {
        iter.into_iter().sum()
    }
    println!("  sum(数组) = {}，sum(vec) = {}", sum(sizes), sum(vec![1, 2, 3]));
    // 警示：arr.into_iter() 方法调用在当前 edition 返回按值迭代（2021 edition 起）；
    //       1.53 当时返回 &T，且曾提示语义将在未来 edition 改变。

    // ============================================================
    println!("\n3. 数值类型的 BITS 关联常量");
    // 1.53 新增：u8::BITS、u32::BITS、usize::BITS 等。
    // 注意：BITS 是关联常量，非方法，不用括号。
    println!(
        "  u8::BITS = {}, u32::BITS = {}, usize::BITS = {}",
        u8::BITS,
        u32::BITS,
        usize::BITS
    );

    // ============================================================
    println!("\n4. Option::insert 与 Duration 系列");
    let mut v: Option<i32> = None;
    let old = v.insert(5); // None 时也“占用位”并写入
    println!("  Option::insert(5) 旧值 = {:?}", old);
    let old2 = v.insert(7); // 旧值被覆盖并返回
    println!("  再 insert(5+2) 旧值 = {:?}", old2);
    use std::time::Duration;
    println!("  Duration::ZERO = {:?}, is_zero = {}", Duration::ZERO, Duration::ZERO.is_zero());
    println!(
        "  saturating_add(1s + 2s) = {:?}",
        Duration::from_secs(1).saturating_add(Duration::from_secs(2))
    );

    // ============================================================
    println!("\n5. (Bound,Bound) 切片索引 与 max_by");
    use std::ops::Bound;
    let data = [5, 6, 7, 8];
    let sub = &data[(Bound::Included(1), Bound::Included(2))];
    println!("  data[(Included(1), Included(2))] = {:?}", sub);
    println!("  max_by 长度最长 = {:?}", ["a", "abc"].iter().max_by_key(|s| s.len()));

    // ============================================================
    println!("\n6. :pat_param / 兼容性：BITS 冲突、IPv4 八进制拒绝");
    println!("  - :pat_param 宏匹配器与 :pat 同义（语义隔离备用）。");
    println!("  - 新增 BITS 常量可能与第三方同名常量冲突；Ipv4Addr::from_str 不再接受八进制。");

    // ============================================================
    println!("\n7. array::IntoIter 对比 by-value 用法");
    // 1.53 起：std::array::IntoIter::new([...]) 显式按值迭代。
    // （1.51 起已稳定；这里展示 1.53 数组 IntoIterator 全打通后的形态）
    let it = std::array::IntoIter::new([1u8, 2, 3]);
    for (i, v) in it.enumerate() {
        println!("  IntoIter[{}] = {}", i, v);
    }
}
