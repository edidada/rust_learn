// rustc 1.62.0 演示 —— 枚举 #[default]、bool::then_some、total_cmp、Mutex futex
fn main() {
    println!("rustc 1.62.0 演示");

    // ============================================================
    println!("\n1. 枚举 #[derive(Default)] + #[default] 变体");
    // 1.62 前：枚举 derive Default 编译失败，需要手写 impl Default。
    // 1.62 起：给一个变体标 #[default]（必须是无字段变体）。
    #[derive(Debug, Default, PartialEq)]
    enum Level {
        Low,
        #[default]
        Medium,
        High,
    }
    let d = Level::default();
    println!("  Level::default() = {:?}", d);
    assert_eq!(d, Level::Medium);

    // ============================================================
    println!("\n2. bool::then_some");
    // 1.62 稳定：then_some(val)；对比 then(|| val) 是闭包惰性求值。
    // then_some 非惰性：val 先求值（注意）
    let active = true;
    println!("  true.then_some(\"on\") = {:?}", active.then_some("on"));
    println!("  false.then_some(1) = {:?}", false.then_some(1));

    // ============================================================
    println!("\n3. f64::total_cmp —— NaN 安全的浮点总序");
    // 1.62 稳定：浮点“全序”比较，NaN 单独可排，解决 sort_by 部分序 NaN 陷阱。
    let mut fs = [3.0f64, -1.0f64, f64::NAN, 2.5f64];
    fs.sort_by(|a, b| a.total_cmp(b));
    println!("  total_cmp 排序（NaN 也有确定位置）= {:?}", fs);

    // ============================================================
    println!("\n4. Linux Mutex/Condvar 改 futex 实现");
    use std::sync::{Condvar, Mutex};
    // 行为 API 不变，仅实现替换（更快、更小的同步基元）。
    let pair = (Mutex::new(false), Condvar::new());
    {
        let mut flag = pair.0.lock().unwrap();
        *flag = true;
        pair.1.notify_all();
    }
    println!("  Mutex 唤醒/通知语义不变（futex 实现）");

    // ============================================================
    println!("\n5. Stdin::lines 与 cargo add");
    println!("  - Stdin::lines 稳定：stdin.lock().lines() 逐行读取（不在无 tty 场景演示）。");
    println!("  - cargo add 稳定：cargo add serde --features derive 一条命令加依赖；");
    println!("    包 ID 支持 name@version 语法。");
}
