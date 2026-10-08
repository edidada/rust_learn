// rustc 1.42.0 演示 —— slice pattern 子切片 / matches! / ManuallyDrop::take / ptr API / condvar
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    println!("rustc 1.42.0 演示");

    // 1.42.0 引入：slice pattern 支持子切片（subslices）。
    // 以前只能在开头匹配字面量、尾部用 rest；现在前部、中部、尾部都能用 `..` 与绑定捕获。
    println!("\n1. slice pattern 子切片");
    fn classify(words: &[&str]) -> String {
        match words {
            // 固定头部 + 中间 rest 捕获
            ["Hello", rest @ ..] => format!("你好 + 剩余 {:?}", rest),
            // 头尾固定，中间捕获
            ["Foo", mid @ .., "Bar"] => format!("Foo ... Bar，夹着 {:?}", mid),
            // 元素长度为 1 的切片
            [only] => format!("单元素 {}", only),
            rest => format!("其余情况 {:?}", rest),
        }
    }
    println!("classify([\"Hello\",\"World\",\"!\"])         = {}", classify(&["Hello", "World", "!"]));
    println!("classify([\"Foo\",\"mid\",\"Bar\"])     = {}", classify(&["Foo", "mid", "Bar"]));
    println!("classify([\"solo\"])                        = {}", classify(&["only"]));
    println!("classify([\"x\",\"y\"])                       = {}", classify(&["x", "y"]));

    // 1.42.0 稳定 matches! —— 一个表达式与模式匹配的断言宏（旧写法 match x { Variant => true, _ => false }）。
    println!("\n2. matches!");
    let n = 7;
    println!("matches!(7, 1..=10) = {}", matches!(n, 1..=10));
    let t = "abc";
    println!("matches!(t, \"abc\") = {}（范围模式仅限 char/数值，&str 不行）", matches!(t, "abc"));

    // 1.42.0 稳定 ManuallyDrop::take —— 取出内部值而绕过 Drop（并允许后面手工避免 double drop）。
    println!("\n3. ManuallyDrop::take");
    let mut md = std::mem::ManuallyDrop::new(String::from("owned by manually drop"));
    let s = unsafe { std::mem::ManuallyDrop::take(&mut md) };
    println!("take 出来的值 = {:?}（原 md 已可被 ManuallyDrop::drop 手工处理）",
        s);

    // 1.42.0 稳定：ptr::slice_from_raw_parts(_mut) —— 由指针+长度合成切片裸指针。
    println!("\n4. ptr::slice_from_raw_parts(_mut)");
    let data = [10u32, 20, 30];
    let p = data.as_ptr();
    let sl: *const [u32] = std::ptr::slice_from_raw_parts(p, data.len());
    println!("slice_from_raw_parts → {:?}（直接把裸指针当切片打印）", unsafe { &*sl });

    // 1.42.0 稳定 CondVar::wait_while —— "条件不满足就一直等"的简洁写法。
    println!("\n5. Condvar::wait_while");
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let p2 = Arc::clone(&pair);
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*p2;
        thread::sleep(std::time::Duration::from_millis(30));
        *lock.lock().unwrap() = true; // 置条件
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair;
    // wait_while：在谓词为 false 时持续等待，谓词为 true 解锁返回
    let mut flag = lock.lock().unwrap();
    while !*flag {
        flag = cvar.wait_while(flag, |f| !*f).unwrap(); // 直接返回 MutexGuard
    }
    println!("wait_while 返回，条件已满足（flag = {}）", *flag);
    handle.join().unwrap();
}
