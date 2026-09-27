// rustc 1.17.0 演示 —— 字段初始化简写 / Cell 非 Copy / Ordering::then / range
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, Bound};

fn main() {
    println!("rustc 1.17.0 演示");

    println!("\n1. 字段初始化简写（RFC 1682）");
    let x = 3;
    let y = 4;
    let p = Point { x, y }; // 当年必须写 x: x, y: y
    println!("Point {{ x, y }} -> ({}, {})", p.x, p.y);

    println!("\n2. Cell 存非 Copy 类型（RFC 1651，1.17 稳定 swap/take 等）");
    let cell = Cell::new(String::from("first"));
    cell.set(String::from("second"));
    let taken = cell.take(); // 取走内容并留下 Default
    println!("take 取走 = {:?}，cell 现在为 {:?}", taken, cell.into_inner());

    println!("\n3. Ordering::then / then_with（多级比较）");
    let people = [("Bo", 30), ("Al", 25), ("Cy", 25)];
    let mut ranked = people;
    ranked.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    println!("按年龄再按名字排序 = {:?}", ranked);
    println!("then_with 演示 = {:?}", Ordering::Equal.then_with(|| Ordering::Less));

    println!("\n4. BTreeMap::range（1.17 稳定）");
    let mut scores = BTreeMap::new();
    for (k, v) in [("a", 60), ("b", 75), ("c", 90)] {
        scores.insert(k, v);
    }
    let mid: Vec<(&str, i32)> = scores
        .range((Bound::Included("a"), Bound::Included("b")))
        .map(|(k, v)| (*k, *v))
        .collect();
    println!("range a..=b = {:?}", mid);

    println!("\n5. Rc::into_raw / from_raw / ptr_eq（1.17 稳定）");
    use std::rc::Rc;
    let rc = Rc::new(42);
    let raw = Rc::into_raw(rc);
    let restored = unsafe { Rc::from_raw(raw) };
    println!("roundtrip 后值 = {}", restored);
    println!("ptr_eq 与克隆体 = {}", Rc::ptr_eq(&restored, &Rc::clone(&restored)));

    println!("\n6. Box<str> 与 String 互转（1.17 稳定一批 Box 转换）");
    let boxed: Box<str> = String::from("boxed-str").into_boxed_str();
    let back: String = boxed.into(); // From<Box<str>> for String
    println!("String -> Box<str> -> String = {}", back);

    println!("\n7. static/const 默认 'static（println 讲解节）");
    // 当年形态：1.17 起 static 与 const 的生命周期默认 'static（RFC 1623），
    // 无需再写 'static 标注；此处静态项演示该基线。
    static GREETING: &str = "hello"; // 生命周期默认 'static
    println!("static GREETING = {}", GREETING);
}

struct Point {
    x: i32,
    y: i32,
}
