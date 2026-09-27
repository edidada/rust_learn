// rustc 1.11.0 演示 —— sum/product 稳定 / split_off / assert_eq! 自定义消息
use std::cell::RefCell;
use std::collections::{BTreeMap, BinaryHeap};
use std::borrow::Cow;
use std::num::Wrapping;

fn main() {
    println!("rustc 1.11.0 演示");

    println!("\n1. Iterator::sum / product（1.11 才稳定，当年需手写 fold）");
    let nums = [1_i32, 2, 3, 4];
    println!("sum = {}, product = {}", nums.iter().sum::<i32>(), nums.iter().product::<i32>());

    println!("\n2. BTreeMap::split_off / append");
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    let mut greater = map.split_off(&2); // >= 2 的键分走（append 需要 &mut Self）
    println!("原 map = {:?}，split_off(&2) = {:?}", map, greater);
    map.append(&mut greater); // 又并回来（签名是 &mut Self，移动元素）
    println!("append 后 map = {:?}", map);

    println!("\n3. Cell::get_mut / RefCell::get_mut（需 &mut 引用，免运行时检查）");
    let mut cell = std::cell::Cell::new(5_u8);
    *cell.get_mut() += 1;
    println!("Cell 经 get_mut 改成 {}", cell.get());
    let mut rc = RefCell::new(vec![1, 2]);
    rc.get_mut().push(3); // get_mut() 返回 RefMut（borrow_mut 等价），仅当无其他借用时才可调用
    println!("RefCell 经 get_mut 改成 {:?}", rc.borrow());

    println!("\n4. BinaryHeap::append");
    let mut h1: BinaryHeap<i32> = [3, 1].into();
    let mut h2: BinaryHeap<i32> = [5, 4].into();
    h1.append(&mut h2); // 签名 &mut Self：元素从 h2 移动进 h1
    println!("合并后 peek = {:?}", h1.peek());

    println!("\n5. assert_eq! 自定义消息（1.11 起对齐 assert!）");
    let got = 4;
    assert_eq!(got, 4, "值应等于 4，实际 {}", got);
    println!("assert_eq! 带消息通过");

    println!("\n6. Cow Default / Wrapping 十六进制 Display");
    let cow: Cow<'static, str> = Cow::default();
    println!("Cow::default() = {:?}", cow);
    println!("Wrapping(255) hex = {:x}", Wrapping(255_u8));
}
