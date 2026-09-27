// rustc 1.19.0 演示 —— loop 返回值 / union / 闭包转 fn / cmp::Reverse / ThreadId
use std::cmp::Reverse;

fn main() {
    println!("rustc 1.19.0 演示");

    println!("\n1. loop 带 break 值（RFC 1624，1.19 稳定）");
    // 当年形态：1.18 及以前 loop 无法直接产出值，要用可变变量绕
    let found = loop {
        let mut i = 0;
        loop {
            i += 1;
            if i * i > 20 {
                break; // 内层循环只是退出（作为语句要求类型为 ()）
            }
        }
        break i; // 外层 loop break 带值：i*i 已 > 20
    };
    println!("loop break 返回值 = {}", found);
    let mut counter = 0;
    let doubled = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("计数到 3 时 break {} -> {}", counter, doubled);

    println!("\n2. 元组结构体数字字段构造（RFC 1506）");
    struct Point(u32, u32);
    let p = Point { 0: 7, 1: 0 }; // 1.19 起可用数字字段名构造
    println!("Point {{ 0: 7, 1: 0 }} -> ({}, {})", p.0, p.1);

    println!("\n3. C 兼容 union（RFC 1444，1.19 稳定）");
    // union 只能含 Copy 类型、不能有 Drop；字段访问需 unsafe（与 C 共用内存布局）
    #[derive(Clone, Copy)]
    union IntOrFloat {
        i: u32,
        f: f32,
    }
    let as_int = IntOrFloat { i: 0x3f800000 };
    // 访问 union 字段是不安全操作：编译器无法保证当前激活的是哪个变体
    let bits = unsafe { as_int.i };
    let as_float = IntOrFloat { f: 1.0 };
    let val = unsafe { as_float.f };
    println!("union 以 u32 读出 = 0x{:x}；以 f32 读出 = {}", bits, val);

    println!("\n4. 非捕获闭包强转 fn 指针（RFC 1558）");
    // 不捕获环境的闭包可直接转成 fn 指针
    let square: fn(i32) -> i32 = |v| v * v;
    println!("fn 指针调用 square(6) = {}", square(6));

    println!("\n5. cmp::Reverse（1.19 稳定，反序排序）");
    let mut nums = vec![3, 1, 4, 1, 5];
    nums.sort_by_key(|&x| Reverse(x));
    println!("Reverse 降序 = {:?}", nums);

    println!("\n6. thread::ThreadId（1.19 稳定）");
    let id = std::thread::current().id();
    println!("当前线程 id = {:?}", id);

    println!("\n7. Command::envs（1.19 稳定）");
    // 批量给子进程设置环境变量；此处只构建命令不做跨平台执行
    let mut cmd = std::process::Command::new("echo"); // 先绑定再链式，避免临时值被丢弃
    cmd.envs([("FOO", "bar"), ("BAZ", "qux")]);
    println!("envs([(\"FOO\",\"bar\"),(\"BAZ\",\"qux\")]) 已挂到子进程命令（未执行）: {:?}", cmd);

    println!("\n8. eprintln!（1.19 加入 prelude）");
    eprintln!("（stderr）eprintln! 与 println! 同用法，但写到标准错误");
    println!("eprintln! 已输出到 stderr");
}
