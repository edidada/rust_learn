// rustc 1.5.0 演示 —— Path 稳定化 / Iterator 比较 / Formatter 控件 / 无损 From
use std::fmt;
use std::path::Path;

fn main() {
    println!("rustc 1.5.0 演示");

    println!("\n1. Vec::resize / slice::split_first / split_last");
    let mut v = vec![1, 2];
    v.resize(5, 7); // 1.5 稳定
    println!("resize(5,7) = {:?}", v);
    let (first, rest) = v.split_first().unwrap();
    let (last, _) = v.split_last().unwrap();
    println!("first = {}, last = {}, 其余 = {:?}", first, last, rest);

    println!("\n2. BinaryHeap::from + into_sorted_vec");
    let heap: std::collections::BinaryHeap<i32> = vec![3, 1, 4, 1, 5].into();
    let sorted = heap.into_sorted_vec(); // 1.5 稳定
    println!("into_sorted_vec = {:?}", sorted);

    println!("\n3. VecDeque::insert / as_slices / shrink_to_fit");
    let mut dq: std::collections::VecDeque<i32> = [2, 3].into();
    dq.insert(0, 1); // 头插
    dq.insert(3, 4);
    println!("deque = {:?}（as_slices = {:?}）", dq, dq.as_slices());
    dq.shrink_to_fit();

    println!("\n4. Iterator::eq / cmp / partial_cmp（1.5 稳定）");
    println!("(0..3).eq(0..3) = {}", (0..3).eq(0..3));
    println!("(0..3).cmp(1..3) = {:?}", (0..3).cmp(1..3));
    println!("(0..3).partial_cmp(2..4) = {:?}", (0..3).partial_cmp(2..4));

    println!("\n5. str::match_indices");
    let hits: Vec<(usize, &str)> = "ababab".match_indices("ab").collect();
    println!("{:?}", hits);

    println!("\n6. Path::exists / is_file / is_dir / canonicalize（1.5 稳定）");
    let p = Path::new("Cargo.toml");
    println!("exists = {}, is_file = {}", p.exists(), p.is_file());
    if let Ok(canon) = p.canonicalize() {
        println!("canonicalize = {}", canon.display());
    }

    println!("\n7. Formatter 的 width/precision/sign 控制");
    struct Price(f64);
    impl fmt::Display for Price {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 这些字段 1.5 才稳定暴露
            f.pad_integral(true, "", &format!("{:.2}", self.0))
        }
    }
    println!("自定义 Display 用 Formatter 控件: {}", Price(9.5));

    println!("\n8. Condvar::wait_timeout 吃 Duration");
    let pair = std::sync::Arc::new((std::sync::Mutex::new(0_u32), std::sync::Condvar::new()));
    let (lock, cvar) = &*pair;
    let guard = lock.lock().unwrap();
    let (guard, _res) = cvar.wait_timeout(guard, std::time::Duration::from_millis(1)).unwrap();
    println!("wait_timeout 超时后值 = {}", *guard);

    println!("\n9. 无损 From 整型↔浮点转换");
    let small: f32 = 200_u8.into(); // u8 → f32 无损
    let big: f64 = 64_u32.into(); // u32 → f64 无损（i64→f64 无 From impl，浮点转换有精度取舍）
    let widen: f64 = 0.5_f32.into();
    println!("u8→f32 = {}, i64→f64 = {}, f32→f64 = {}", small, big, widen);
}
