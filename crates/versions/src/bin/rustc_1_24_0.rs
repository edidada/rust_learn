// rustc 1.24.0 演示 —— const-fn 化的 Atomic::new + RefCell::replace/swap
// 该版本 introduces:
// 1) 一批"构造函数"成为 const fn：Atomic*::new、Cell::new、mem::size_of 等可用于 const 上下文。
//    旧写法（1.24 前）：static 计数器必须用 unstable 的构造，或运行时初始化。
// 2) RefCell::replace / RefCell::swap 稳定。
// 3) 浮点 Debug 输出固定带小数点（行为变化，如 2.0f64 的 {:?} 现在是 "2.0"）。

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

// 1.24 起静态初始化原子计数器是稳定 const fn（此前 AtomicUsize::INIT（历史形态）已不存在）
static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn main() {
    println!("rustc 1.24.0 演示");

    // 1. const-fn 化的 Atomic*::new：static 可直接初始化
    println!("\n1. static 原子变量 = AtomicUsize::new(1)（const fn）");
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("COUNTER 初始 1，fetch_add 后 load = {}", COUNTER.load(Ordering::SeqCst));

    // 2. mem::size_of 参与 const 表达式
    println!("\n2. size_of::<usize>() 做数组长度（const 上下文）");
    let buffer: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
    println!("buffer 长度 = {}（= usize 的字节数）", buffer.len());

    // 3. RefCell::replace / RefCell::swap
    println!("\n3. RefCell::replace / swap");
    let c = RefCell::new(10);
    let old = c.replace(20); // 返回旧值
    println!("replace(20) 返回旧值 {}", old);
    let d = RefCell::new(99);
    RefCell::swap(&c, &d); // 1.24 稳定：直接以 RefCell 为单位交换
    println!("swap 后 c={:?} d={:?}", c, d);

    // 4. 浮点 Debug 行为变化：一定带小数点
    println!("\n4. 浮点 Debug 带小数点");
    println!("2.0f64 的 {:?} = {:?}", 2.0f64, 2.0f64);
    println!("3.5f32 的 {:?} = {:?}", 3.5f32, 3.5f32);

    // 5. atomic::spin_loop_hint：自旋等待提示
    println!("\n5. spin_loop_hint（现名 hint::spin_loop）");
    let mut spins = 0;
    let target = AtomicUsize::new(10);
    while target.load(Ordering::Relaxed) > 5 {
        // 自旋等不会真发生，用 while 演示循环等待姿态
        target.fetch_sub(1, Ordering::Relaxed);
        spins += 1;
    }
    println!("自旋 {} 次后值 = {}", spins, target.load(Ordering::Relaxed));
}
