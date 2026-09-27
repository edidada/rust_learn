// rustc 1.66.0 演示 —— 枚举显式判别值、..=X 模式、BTreeMap first/last、整数溢出 API 家族
fn main() {
    println!("rustc 1.66.0 演示");

    // ============================================================
    println!("\n1. repr(Int) 枚举带数据变体的显式判别值");
    // 1.66 起，#[repr(Int)] 枚举即使变体带字段也能写显式判别值：
    // 以前带字段变体不能指定 discriminant，现在允许了。
    #[repr(u8)]
    #[allow(dead_code)]
    enum Mixed {
        A(u8) = 0,
        B(i8) = 1,
        C(bool) = 42,
    }
    let v = Mixed::C(true);
    // 取判别值需要查看表示；这里只演示构造合法性
    println!("  Mixed::C(true) 构造成功，discriminant = 42（按声明）");
    let _ = v;

    // ============================================================
    println!("\n2. 模式中的开放式范围 ..=X");
    // 1.66 稳定在模式里写 ..=X（另一端开放），常用于匹配上界。
    fn grade(n: i32) -> &'static str {
        match n {
            ..=59 => "不及格",
            60..=84 => "及格",
            _ => "优秀",
        }
    }
    for n in [55, 70, 95] {
        println!("  {} -> {}", n, grade(n));
    }

    // ============================================================
    println!("\n3. BTreeMap / BTreeSet 的 first / last / pop 系列");
    let mut m: std::collections::BTreeMap<&str, i32> = std::collections::BTreeMap::new();
    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 3);
    println!("  first_key_value = {:?}", m.first_key_value()); // Some(("a", 1))
    println!("  last_key_value  = {:?}", m.last_key_value()); // Some(("c", 3))
    let mut s: std::collections::BTreeSet<i32> = [10, 3, 7].into();
    println!("  pop_first = {:?}, pop_last = {:?}", s.pop_first(), s.pop_last());
    println!("  剩余集合 = {:?}", s); // {7}

    // ============================================================
    println!("\n4. 无符号/带符号混合运算 API（uX/iX 一族）");
    // checked_add_signed: u32 + i32 可为负，下溢返回 None
    println!("  5u32.checked_add_signed(-7) = {:?}", 5u32.checked_add_signed(-7)); // None
    println!("  5u32.checked_add_signed(-3) = {:?}", 5u32.checked_add_signed(-3)); // Some(2)
    println!("  3i32.checked_add_unsigned(4u32) = {:?}", 3i32.checked_add_unsigned(4u32));
    println!("  1i32.saturating_sub_unsigned(5u32) = {}", 1i32.saturating_sub_unsigned(5u32));

    // ============================================================
    println!("\n5. 其他稳定 API");
    // Option::unzip
    let oz = Some((1, "a"));
    let (oa, ob) = oz.unzip();
    println!("  Option::unzip -> ({:?}, {:?})", oa, ob); // (Some(1), Some("a"))
    // Duration::try_from_secs_f64：浮点不能精确表示或溢出则 Err
    println!("  Duration::try_from_secs_f64(1.5) = {:?}", std::time::Duration::try_from_secs_f64(1.5));
    println!("  Duration::try_from_secs_f64(-1.0) = {:?}", std::time::Duration::try_from_secs_f64(-1.0));
    // core::hint::black_box：阻止编译器把常量折叠/消除
    let x = std::hint::black_box(3);
    println!("  black_box(3) * 2 = {}", std::hint::black_box(x * 2));
    // TryFrom<Vec<T>> for Box<[T; N]>：长度不匹配则 Err
    let v: Vec<i32> = vec![1, 2, 3];
    let arr: Result<Box<[i32; 3]>, _> = v.clone().try_into();
    println!("  Vec -> Box<[i32; 3]> = {:?}", arr.as_deref());
    let bad: Result<Box<[i32; 2]>, _> = v.try_into();
    println!("  Vec[3] -> Box<[i32; 2]> 长度不符 = {:?}", bad.err().is_some());

    // ============================================================
    println!("\n6. 行为变更说明");
    println!("  - 常量求值错误从 deny lint 变为硬错误（编译期直接报错）");
    println!("  - Command 派生子进程默认继承父进程信号掩码");
    println!("  - 新增 cargo remove 子命令");
}
