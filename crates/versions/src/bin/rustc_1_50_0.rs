// rustc 1.50.0 演示 —— 数组重复表达式用 const、bool::then、clamp、slice::fill
fn main() {
    println!("rustc 1.50.0 演示");

    // ============================================================
    println!("\n1. 数组重复表达式 [x; N] 的 x 可为 const 常量");
    // 1.50 确认：`[x; N]` 中的 x 可以写具名 const。
    // 注：这是 const 值的“值语义”重复——每个元素都是拷贝，不是引用。
    // 对比旧行为：以前 const 重复表达式必须写常量名字面量本身。
    const NAN: u8 = 3;
    let a = [NAN; 10];
    println!("  [const NAN; 10] 首元素 = {}", a.first().unwrap());
    // 非常量重复是老能力：[0u8; 5]
    let b = [0u8; 5];
    println!("  [0u8; 5] 长度 = {}", b.len());

    // ============================================================
    println!("\n2. bool::then —— 条件成立才计算/产出 Some");
    // 1.50 稳定 bool::then：true -> Some(f())，false -> None（惰性求值）。
    // 对比旧写法：if x > 0 { Some(x * 2) } else { None }
    let x: i32 = 3; // 显式标注：is_positive/is_negative 是具体整型方法
    let v = x.is_positive().then(|| x * 2);
    println!("  x.is_positive().then(|| x*2) -> {:?}", v);
    let v2 = x.is_negative().then(|| x * 2);
    println!("  x.is_negative().then(...)  -> {:?}", v2);
    // Entry::or_insert_with_key：空位时按 key 生成默认值（本版稳定）。
    use std::collections::HashMap;
    let mut m: HashMap<&str, usize> = HashMap::new();
    m.entry("a").or_insert_with_key(|k| k.len());
    println!("  Entry::or_insert_with_key(\"a\") -> {:?}", m.get("a"));

    // ============================================================
    println!("\n3. clamp 系列：Ord::clamp / f64::clamp");
    // 1.50 稳定 clamp：把值限制在 [low, high] 区间内。
    // 对比旧写法：x.max(low).min(high)
    println!("  5.clamp(1, 3) = {}", 5.clamp(1, 3));
    println!("  2.7f64.clamp(0.0, 2.0) = {}", 2.7f64.clamp(0.0, 2.0));

    // ============================================================
    println!("\n4. slice::fill 与 RefCell::take");
    // 1.50 稳定 slice::fill：用一个值填充整个切片。
    // 对比旧写法：for e in buf.iter_mut() { *e = 7; }
    let mut buf = vec![0u8; 4];
    buf.fill(7);
    println!("  buf.fill(7) -> {:?}", buf);
    // RefCell::take：取出内部值，留下 Default::default()。
    use std::cell::RefCell;
    let c = RefCell::new(String::from("hi"));
    let got = c.take();
    println!("  RefCell::take 取出 = {:?}，剩余 = {:?}", got, c.borrow());
    // 提醒：ManuallyDrop<T> union 字段赋值现在视为安全（见笔记）。

    // ============================================================
    println!("\n5. 兼容性提醒（println 讲解，不演示弃用 API）");
    println!("  - 原子类型的 compare_and_swap 已弃用，用 compare_exchange_weak");
}

// 本版附带库级变化提示（编译期即可见）：
//   Index/IndexMut 对任意长度数组实现；File 的 Option<File> 大小与 File 相同（Unix niche）。
