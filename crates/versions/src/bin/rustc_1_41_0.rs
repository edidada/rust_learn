// rustc 1.41.0 演示 —— map_or/else / 嵌套 receiver / NonZero 窄化 From / Weak 计数 / MaybeUninit Debug
use std::num::{NonZeroU16, NonZeroU8};
use std::rc::{Rc, Weak};

fn main() {
    println!("rustc 1.41.0 演示");
    // 1.41.0 稳定：Result::map_or / map_or_else —— 相当于 map(...).unwrap_or(...) 的组合捷径。
    println!("\n1. Result::map_or / map_or_else");
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("bad input");
    println!("Ok(5).map_or(0, |v| v*2)        = {}", ok.map_or(0, |v| v * 2));
    println!("Err(..).map_or(0, |v| v*2)     = {}", err.map_or(0, |v| v * 2));
    println!("Err(..).map_or_else(|e| e.len() as i32, |v| v) = {}",
        err.map_or_else(|e| e.len() as i32, |v| v));

    // 1.41.0 引入：self 位置的 receiver 可以任意嵌套（如 Box<Box<Self>>），
    // 此前只允许 Self、&Self、&mut Self、Arc<Self>、Rc<Self>、Box<Self>。
    println!("\n2. 嵌套 receiver（self: Box<Box<Self>>）");
    struct Node {
        v: u8,
    }
    impl Node {
        // 1.41 之前这种嵌套 receiver 不能出现在方法定义里
        fn depth(self: Box<Box<Self>>) -> u8 {
            self.v
        }
    }
    let n = Node { v: 7 };
    println!("Box<Box<Node>>::depth() = {}", Box::new(Box::new(n)).depth());

    // 1.41.0 引入：NonZero* 类型之间窄位宽的 From 转换（NonZeroU16: From<NonZeroU8> 等）。
    println!("\n3. NonZero 窄化 From");
    let nz8 = NonZeroU8::new(3).unwrap();
    let nz16 = NonZeroU16::from(nz8); // 1.41 起可安全转换，彼岸仍非零
    println!("NonZeroU16::from(NonZeroU8(3)) = {}", nz16.get());

    // 1.41.0 稳定：std::rc::Weak::{strong_count, weak_count}。
    println!("\n4. Weak::strong_count / weak_count");
    let strong = Rc::new(1);
    let weak: Weak<i32> = Rc::downgrade(&strong);
    println!("strong_count = {}, weak_count = {}（仅弱引用）",
        weak.strong_count(), weak.weak_count());
    let strong2 = Rc::clone(&strong);
    println!("再 clone 一个强引用后：strong_count = {}", weak.strong_count());
    drop(strong2);
    drop(strong);
    println!("全部强引用释放后：strong_count = {}，weak 还活着，读数据失败为 {:?}",
        weak.strong_count(), weak.upgrade().is_none());

    // 1.41.0 引入：MaybeUninit<T> 实现 fmt::Debug（可打印"未初始化状态"）。
    println!("\n5. MaybeUninit 的 Debug");
    let mu: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::new(42);
    println!("MaybeUninit::new(42) Debug 形式 = {:?}", mu);
}
