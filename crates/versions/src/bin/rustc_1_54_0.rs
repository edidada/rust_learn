// rustc 1.54.0 演示 —— 属性宏取值、map into_keys/values、VecDeque 二分
fn main() {
    println!("rustc 1.54.0 演示");

    // ============================================================
    println!("\n1. 内置属性中调用宏：#[doc = include_str!(\"README.md\")]");
    // 1.54 稳定：#![doc = 宏()] 之类“属性值 = 宏展开”。
    // 常见用法是把 README 当 crate 级文档（本仓库无 README 时不演示副作用）。
    println!("  典型写法：#![doc = include_str!(\"../README.md\")]");
    println!("  意义：文档内容可来自宏展开（include_str!/concat!），不必拷贝字符串。");

    // ============================================================
    println!("\n2. Map 的 into_keys / into_values");
    use std::collections::BTreeMap;
    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    // 1.54 稳定：消耗 map，得到所有 key / 所有 value 的迭代器
    println!("  BTreeMap::into_keys -> {:?}", map.clone().into_keys().collect::<Vec<_>>());
    println!("  BTreeMap::into_values -> {:?}", map.into_values().collect::<Vec<_>>());

    // ============================================================
    println!("\n3. VecDeque::binary_search / partition_point");
    use std::collections::VecDeque;
    let dq: VecDeque<i32> = [1, 3, 5, 7, 9].into_iter().collect();
    println!("  binary_search(5) -> {:?}", dq.binary_search(&5));
    let p = dq.partition_point(|&x| x < 7);
    println!("  partition_point(x<7) = {}", p);

    // ============================================================
    println!("\n4. ErrorKind::OutOfMemory");
    use std::io::ErrorKind;
    // 1.54 新增错误分类；实际分配失败会报此 Kind（此处只演示枚举存在）。
    let e: ErrorKind = ErrorKind::OutOfMemory;
    println!("  ErrorKind::OutOfMemory -> {:?}", e);

    // ============================================================
    println!("\n5. impl Trait 多生命周期（println 讲解）");
    println!("  1.54 前：impl Trait<'a,'b> 需要 'b: 'a 的显式 outlive 约束。");
    println!("  1.54 起：多个相互独立的生命周期也可用，减少无谓的 bound。");
    let _ = 0;
}
