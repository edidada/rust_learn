// rustc 1.3.0 演示 —— Duration 稳定化 / Error 下转型 / two-way 搜索
use std::time::Duration;

fn main() {
    println!("rustc 1.3.0 演示");

    println!("\n1. std::time::Duration 稳定化（1.3 首次稳定）");
    // 当年形态：Duration 的 from_secs/from_millis 与 today 相同；此前 std 里只有毫秒版定时 API
    let d = Duration::from_secs(2) + Duration::from_millis(500);
    println!("Duration = {:?}（2.5s，此后 thread::sleep 等都吃 Duration）", d);

    println!("\n2. Instant::elapsed 计时");
    let start = std::time::Instant::now();
    let mut total = 0_u64;
    for i in 1..=1000 {
        total += i;
    }
    let el = start.elapsed();
    println!("1..=1000 累加 = {}，耗时 = {:?}", total, el);

    println!("\n3. Error 下转型 downcast_ref（1.3 稳定）");
    use std::error::Error;
    let e = parse_int("10x").unwrap_err();
    // 动态地把 trait 对象还原为具体错误类型
    let as_conversion = e.downcast_ref::<std::num::ParseIntError>();
    match as_conversion {
        Some(p) => println!("下转型到 ParseIntError: {}", p),
        None => println!("不是 ParseIntError"),
    }

    println!("\n4. two-way 搜索提速的常用方法");
    let text = "rust-lang-rust";
    println!("contains(\"lang\") = {}", text.contains("lang"));
    println!("find(\"lang\") = {:?}", text.find("lang"));
    println!("split('-') = {:?}", text.split('-').collect::<Vec<&str>>());
    println!("starts_with(\"rust\") = {}, ends_with(\"rust\") = {}", text.starts_with("rust"), text.ends_with("rust"));

    println!("\n5. join 取代 connect（切片拼接）");
    let parts = ["a", "b", "c"];
    println!("[\"a\",\"b\",\"c\"].join(\"-\") = {}", parts.join("-"));

    println!("\n6. CString/CStr 的 Borrow/ToOwned 与 Debug（println 讲解节）");
    // 当年形态（1.3 稳定）：CString 实现 Borrow<CStr>，CStr 实现 ToOwned<Owned=CString>，
    // 这样在泛型里就能互相借/克隆；CStr 与 AtomicPtr 也实现 Debug。
    // 这些 API 至今仍在 std 中，此处直接演示：
    let owned = std::ffi::CString::new("hello").unwrap();
    let borrowed: &std::ffi::CStr = owned.as_c_str();
    println!("CString->CStr 借用成功：{:?}（Borrow/ToOwned 一般化于 trait bound 场景）", borrowed);
}

fn parse_int(s: &str) -> Result<i32, Box<dyn Error>> {
    Ok(s.parse::<i32>()?)
}
