// rustc 1.6.0 演示 —— no_std 稳定化 / read_exact / drain 家族 / *_ms API 弃用
use std::collections::HashMap;

fn main() {
    println!("rustc 1.6.0 演示");

    println!("\n1. Read::read_exact（1.6 稳定，读不满则报错）");
    let mut src: &[u8] = b"hello world"; // Read for &[u8]：read_exact 需要 &mut self
    let mut buf = [0_u8; 5];
    src.read_exact(&mut buf).unwrap();
    println!("read_exact 读到 {:?}", String::from_utf8_lossy(&buf));

    println!("\n2. drain 家族：Vec / String / HashMap");
    let mut v = vec![1, 2, 3, 4];
    let drained: Vec<i32> = v.drain(..2).collect();
    println!("Vec drain(..2) = {:?}，剩余 {:?}", drained, v);
    let mut s = String::from("abcdef");
    let cut: String = s.drain(..3).collect();
    println!("String drain(..3) = {:?}，剩余 {:?}", cut, s);
    let mut m = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);
    let kv: Vec<(&str, i32)> = m.drain().collect();
    println!("HashMap drain = {:?}", kv);

    println!("\n3. Iterator::max_by_key / min_by_key（1.6 由 min_by/max_by 改名而来）");
    let words = ["pear", "fig", "banana"];
    let max = words.iter().max_by_key(|w| w.len()).unwrap();
    let min = words.iter().min_by_key(|w| w.len()).unwrap();
    println!("最长 = {:?}，最短 = {:?}", max, min);

    println!("\n4. 毫秒版定时 API 弃用 → Duration 版");
    // 当年形态：thread::sleep_ms(1)、Condvar::wait_timeout_ms(1)（1.6 弃用）
    // 现行等价物：thread::sleep(Duration::from_millis(1))
    std::thread::sleep(std::time::Duration::from_millis(1));
    println!("sleep 1ms 通过 thread::sleep(Duration) 完成");

    println!("\n5. From<T> for Box / Rc / Arc");
    let b: Box<i32> = Box::from(42_i32); // 1.6 起从值直接装箱
    let r: std::rc::Rc<String> = std::rc::Rc::from("shared".to_string());
    let a: std::sync::Arc<u8> = std::sync::Arc::from(1_u8);
    println!("Box = {}, Rc = {}, Arc = {}", b, r, *a);

    println!("\n6. Vec::extend_from_slice（改名自 push_all）");
    let mut v2 = vec![1, 2];
    v2.extend_from_slice(&[3, 4]);
    println!("{:?}", v2);

    println!("\n7. no_std / core 说明（println 讲解节）");
    // 当年形态：#![no_std] 于 1.6 稳定——crate 只依赖 core，
    // core 有基础类型与 trait 但无平台依赖，用于内核/嵌入式。
    // 演示程序本身仍用 std，仅以输出说明该机制的存在与用途。
    println!("#![no_std]：去掉 std，只链接 core（1.6 稳定）；嵌入式/OS 开发的基础。");
}

use std::io::Read;
