// rustc 1.12.0 演示 —— recv_timeout / peek_mut / format! 多格式 / MIR
use std::collections::{BinaryHeap, VecDeque};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    println!("rustc 1.12.0 演示");

    println!("\n1. Receiver::recv_timeout / RecvTimeoutError（1.12 稳定）");
    let (tx, rx) = mpsc::channel::<i32>();
    tx.send(7).unwrap();
    println!("recv_timeout 有值 = {:?}", rx.recv_timeout(Duration::from_millis(10)));
    drop(tx); // 发送端关闭
    let empty = rx.recv_timeout(Duration::from_millis(10));
    println!("断开后 recv_timeout = {:?}", empty); // Disconnected

    println!("\n2. BinaryHeap::peek_mut（1.12 稳定，可原地改堆顶）");
    let mut heap: BinaryHeap<i32> = [5, 3, 8].into();
    if let Some(mut top) = heap.peek_mut() {
        *top = 1; // 把堆顶改成 1，堆会自动下沉重排
    }
    println!("改堆顶后 peek = {:?}", heap.peek());

    println!("\n3. format! 允许同一参数多种风格（1.12 起）");
    println!("\"{0} {0:x} {0:b}\" of 42 = {}", 42);

    println!("\n4. String 实现 AddAssign");
    let mut s = String::from("foo");
    s += "bar";
    println!("s += \"bar\" -> {}", s);

    println!("\n5. VecDeque::contains / LinkedList::contains");
    let dq: VecDeque<i32> = [1, 2, 3].into();
    println!("contains(&2) = {}", dq.contains(&2));

    println!("\n6. Option::from 与 MIR / 新错误格式（println 讲解节）");
    let opt = Option::from(9);
    println!("Option::from(9) = {:?}", opt);
    // 当年形态：1.12 起 rustc 走 MIR 中间表示翻译到 LLVM IR；
    // 错误输出改为新版高亮格式，并可用 --error-format=json 给 IDE 消费。
    println!("1.12：MIR 翻译落地 + 全新错误格式与 JSON 错误输出。");
}
