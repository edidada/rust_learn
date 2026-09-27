// rustc 1.14.0 演示 —— .. 剩余字段模式 / println!() / Wrapping 运算符
use std::borrow::Cow;
use std::num::Wrapping;

fn main() {
    println!("rustc 1.14.0 演示");

    println!("\n1. `..` 匹配其余元组字段（RFC 1492，1.14 稳定）");
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let p = Point { x: 1, y: 2, z: 3 };
    let msg = match p {
        Point { x, .. } if x == 0 => "原点线上",
        Point { x: 1, .. } => "x=1，其余忽略",
        Point { .. } => "其他点",
    };
    println!("结构体模式 Point {{ x: 1, .. }} -> {}", msg);
    let t = (1, 2, 3, 4);
    match t {
        (1, ..) => println!("元组模式 (1, ..) 命中"),
        _ => println!("未命中"),
    }

    println!("\n2. println!() 空参数即打印换行（1.14 起）");
    println!("上一行内容");
    println!(); // 当年必须 println!("") 才能空一行
    println!("上面这一空行来自 println!()");

    println!("\n3. Wrapping 实现完整运算符与 Sum/Product");
    let ws: Wrapping<u8> = [250_u8, 10].iter().map(|&x| Wrapping(x)).sum();
    println!("Wrapping 求和 250+10 = {}", ws); // 回绕成 4
    let prod = Wrapping(16_u8) * Wrapping(16_u8);
    println!("Wrapping 16*16 = {}", prod);

    println!("\n4. From<Cow<str>> for String");
    let borrowed: Cow<'static, str> = Cow::Borrowed("no-copy");
    let owned: Cow<'static, str> = Cow::Owned(String::from("heap"));
    println!("String::from(Cow) = {:?} / {:?}", String::from(borrowed), String::from(owned));

    println!("\n5. HashMap 内存布局优化（println 讲解节）");
    // 当年形态：1.14 重排 HashMap 桶布局，迭代/查找更缓存友好；
    // 且不再复用随机种子，32 位平台更省内存。API 未变，仅性能行为变化。
    let mut m = std::collections::HashMap::new();
    m.insert(1_u8, "one");
    println!("API 不变，仅内部布局优化：{{1: {:?}}}", m[&1]);

    println!("\n6. rustup 与 WebAssembly 实验（println 讲解节）");
    // 当年形态：1.14 起 rustup 是官方推荐安装方式；
    // 并提供 wasm32-unknown-emscripten 实验目标（已知缺陷多）。
    println!("rustup 推荐安装；WASM 实验目标上线。");
}
