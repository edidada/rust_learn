// rustc 1.15.0 演示 —— min_by/max_by / try_iter / 空元组结构体 / Take::into_inner
use std::io::{Cursor, Read, Take};
use std::sync::mpsc;

fn main() {
    println!("rustc 1.15.0 演示");

    println!("\n1. Iterator::min_by / max_by（1.15 稳定，非 key 比较版）");
    let words = ["pear", "fig", "banana"];
    let max = words.iter().copied().max_by(|a, b| a.len().cmp(&b.len()));
    let min = words.iter().copied().min_by(|a, b| a.len().cmp(&b.len()));
    println!("max_by 最长 = {:?}，min_by 最短 = {:?}", max, min);

    println!("\n2. Receiver::try_iter（1.15 稳定，非阻塞迭代）");
    let (tx, rx) = mpsc::channel::<i32>();
    for i in 1..=3 {
        tx.send(i).unwrap();
    }
    drop(tx);
    let got: Vec<i32> = rx.try_iter().collect();
    println!("try_iter 立即取走 = {:?}", got);

    println!("\n3. Rc::strong_count / weak_count（1.15 稳定）");
    use std::rc::{Rc, Weak};
    let strong = Rc::new(5);
    let weak: Weak<i32> = Rc::downgrade(&strong);
    println!("strong = {}, weak = {}", Rc::strong_count(&strong), Rc::weak_count(&strong));

    println!("\n4. 空元组结构体与花括号实例化（RFC 1506，1.15 起）");
    struct Token();
    let t = Token {}; // 当年新允许：用花括号实例化单元/空元组结构体
    let _u = Token();
    println!("struct Token(); 既可 Token() 也可 Token {{}}（实例存在：{:?}）", std::mem::size_of_val(&t));

    println!("\n5. io::Take::into_inner（1.15 稳定）");
    let reader = Cursor::new(b"abcdef" as &[u8]); // 显式成 &[u8]，Take<Cursor<&[u8]>> 才与推断一致
    let mut limited: Take<Cursor<&[u8]>> = reader.take(3);
    let mut buf = [0_u8; 3];
    limited.read_exact(&mut buf).unwrap();
    let inner = limited.into_inner();
    println!("take(3) 读到 {:?}，底层游标位置 = {}", String::from_utf8_lossy(&buf), inner.position());

    println!("\n6. macros 1.1（println 讲解节）");
    // 当年形态：1.15 稳定自定义 derive 的过程宏（RFC 1681），
    // 使 serde::Serialize 之类的 #[derive(...)] 宏可用。
    // 过程宏必须放在独立的 proc-macro crate 中，单文件 std-only 演示无法呈现。
    println!("macros 1.1：#[derive(MyTrait)] 由 proc-macro crate 生成代码（Serde/Diesel 得以人性化使用）。");

    println!("\n7. 排序算法重写（println 讲解节）");
    // 当年形态：标准库排序从朴素归并改为类 Timsort 混合排序，性能大涨。
    let mut v = vec![5, 2, 8, 1];
    v.sort();
    println!("v.sort() = {:?}（底层已是更快的新实现）", v);
}
