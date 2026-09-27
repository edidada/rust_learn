// rustc 1.26.0 演示 —— ..= 范围 / main->Result / 返回位 impl Trait / u128 / 切片模式
// 该版本 introduces:
// 1) `..=` 含尾范围稳定（此前写 `..=10` 是错误，只能 `0..10` 再补边界处理）。
// 2) `main` 可返回 Result：错误会以 Debug 打印并使进程非零退出。
// 3) 返回位置的 impl Trait 稳定：`fn f() -> impl Iterator`。
// 4) u128/i128；固定长度切片匹配；`'_` 匿名生命周期。
// 注：本文件 main 保持统一形态，"main -> Result" 用独立函数 + println 讲解语义。

// 返回位置的 impl Trait：调用方无需知道具体迭代器类型
fn evens() -> impl Iterator<Item = u32> {
    (0..=10).filter(|x| x % 2 == 0)
}

// 模拟 "main -> Result" 的退出语义
fn checked_run(v: i32) -> Result<(), String> {
    if v < 0 {
        Err(format!("v 必须非负，收到 {}", v)) // 真实 main 中会 Debug 打印后非零退出
    } else {
        println!("checked_run 成功 v = {}", v);
        Ok(())
    }
}

// '_ 匿名生命周期：impl 头里可省略命名生命周期
struct Reader<'a> {
    buf: &'a [u8],
}
impl Reader<'_> {
    fn first(&self) -> Option<u8> {
        self.buf.first().copied()
    }
}

fn main() {
    println!("rustc 1.26.0 演示");

    // 1. ..= 含尾范围
    println!("\n1. ..= 稳定");
    let sum: i32 = (0..=3).sum();
    println!("(0..=3).sum() = {}（含 3）", sum);

    // 2. main -> Result<(), E: Debug>
    println!("\n2. main -> Result<(), E: Debug>");
    let _ = checked_run(7);
    let err = checked_run(-1);
    println!("演示返回 Err: {:?}（真实 main 中会打印 stderr 且退出码非 0）", err);

    // 3. 返回位置的 impl Trait
    println!("\n3. -> impl Iterator");
    println!("evens() 前 4 个 = {:?}", evens().take(4).collect::<Vec<_>>());

    // 4. u128 / i128
    println!("\n4. 128 位整数");
    println!("u128::MAX = {}", u128::MAX);
    println!("i128 = {}", -7i128);

    // 5. 固定长度切片模式
    println!("\n5. 切片固定模式");
    let points = [1, 2, 3, 4];
    match points {
        [1, 2, 3, 4] => println!("All points were sequential."),
        _ => println!("Not all points were sequential."),
    }

    // 6. '_ 匿名生命周期
    println!("\n6. '_ 匿名生命周期");
    let r = Reader { buf: b"data" };
    println!("Reader.first() = {:?}", r.first());

    // 7. Entry::and_modify
    println!("\n7. Entry::and_modify");
    let mut m: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    m.entry("k").and_modify(|e| *e += 5).or_insert(0);
    m.entry("k").and_modify(|e| *e += 5).or_insert(0);
    println!("map = {:?}", m);

    // 8. slice::rotate_left / Option::cloned / Box::leak / process::id
    println!("\n8. rotate_left / cloned / Box::leak");
    let mut a = [1, 2, 3, 4, 5];
    a.rotate_left(2);
    println!("rotate_left(2) = {:?}", a);
    let opts = vec![Some(2), None, Some(9)];
    let cloned: Vec<i32> = opts.iter().cloned().filter_map(|x| x).collect();
    println!("Option::cloned 过滤后 = {:?}", cloned);
    let leaked: &'static str = Box::leak("泄漏成 static".to_string().into_boxed_str());
    println!("Box::leak -> {}", leaked);
    println!("process::id() = {}", std::process::id());
}
