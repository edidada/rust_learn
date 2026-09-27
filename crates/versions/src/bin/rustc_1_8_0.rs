// rustc 1.8.0 演示 —— += 重载 / Instant / SystemTime / Ref::map
use std::cell::RefCell;
use std::cell::Ref;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    println!("rustc 1.8.0 演示");

    println!("\n1. AddAssign：让 += 作用于自定义类型（1.8 稳定化）");
    #[derive(Debug)]
    struct Meters(f64);
    impl std::ops::AddAssign for Meters {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }
    let mut d = Meters(1.5);
    d += Meters(2.5);
    println!("1.5m += 2.5m -> {:?}", d);

    println!("\n2. Instant::now / duration_since / elapsed（1.8 稳定）");
    let start = Instant::now();
    let mut acc = 0_u64;
    for i in 0..100_000 {
        acc = acc.wrapping_add(i);
    }
    println!("循环完成 acc={}，elapsed = {:?}", acc, start.elapsed());

    println!("\n3. SystemTime 与 UNIX_EPOCH");
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(since_epoch) => println!("距 UNIX 纪元 = {:?} 秒", since_epoch),
        Err(e) => println!("SystemTimeError: {}", e.duration().as_secs()),
    }

    println!("\n4. Ref::map：对 RefCell 借用内容做投影");
    let cell = RefCell::new(vec![10, 20, 30]);
    let first: Ref<i32> = Ref::map(cell.borrow(), |v| &v[0]);
    println!("vec 第一个元素 = {}", *first);

    println!("\n5. 空结构体花括号形式");
    struct Marker {}
    let _m = Marker {}; // 1.8 起 struct Foo {} 与 struct Foo; 皆可
    println!("struct Marker {{}} 定义并实例化成功");

    println!("\n6. Instant += Duration");
    let mut future = Instant::now();
    future += Duration::from_millis(50);
    println!("+50ms 后的时间点 = {:?}", future);
}
