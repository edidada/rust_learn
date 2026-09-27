// rustc 1.7.0 演示 —— checked/overflowing 运算 / strip_prefix / RandomState
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher, RandomState};
use std::path::Path;

fn main() {
    println!("rustc 1.7.0 演示");

    println!("\n1. 整型 checked / saturating / overflowing 运算（1.7 补齐）");
    // 默认 + - * 在 debug 下溢出 panic；需要溢出语义时用这批显式方法
    println!("checked_add: 5 + 5 = {:?}, MAX + 1 = {:?}", 5_i32.checked_add(5), i32::MAX.checked_add(1));
    println!("saturating_mul: 200_u8 * 2 = {}", 200_u8.saturating_mul(2));
    println!("overflowing_add: 255u8 + 1 = {:?}（值与是否溢出）", 255_u8.overflowing_add(1));
    println!("overflowing_mul: 100000i32 * 100000 = {:?}", 100_000_i32.overflowing_mul(100_000));
    println!("checked_neg: i32::MIN = {:?}", i32::MIN.checked_neg());

    println!("\n2. Path::strip_prefix（1.7 稳定，改名自 relative_from）");
    let full = Path::new("/usr/local/share/doc");
    match full.strip_prefix("/usr/local") {
        Ok(rest) => println!("strip_prefix 成功 -> {}", rest.display()),
        Err(e) => println!("失败：{}（StripPrefixError）", e),
    }
    println!("不匹配时: {:?}", full.strip_prefix("/opt").is_err());

    println!("\n3. Ipv4Addr 分类方法（1.7 稳定）");
    use std::net::Ipv4Addr;
    let lo = Ipv4Addr::new(127, 0, 0, 1);
    let pr = Ipv4Addr::new(192, 168, 1, 1);
    let bc = Ipv4Addr::new(255, 255, 255, 255);
    println!("127.0.0.1 is_loopback = {}", lo.is_loopback());
    println!("192.168.1.1 is_private = {}", pr.is_private());
    println!("255.255.255.255 is_broadcast = {}", bc.is_broadcast());

    println!("\n4. RandomState / BuildHasher（1.7 稳定）");
    // 默认 HashMap 的键种子工厂；也可用 with_hasher 换自定义 BuildHasher
    let m: HashMap<String, i32> = HashMap::with_hasher(RandomState::new());
    let mut h = RandomState::new().build_hasher();
    "seed".hash(&mut h);
    println!("带 RandomState 的 HashMap 建好：{} 项；hash 值 = {}", m.len(), h.finish());

    println!("\n5. CString::into_string（FFI 字符串转 Rust String）");
    use std::ffi::CString;
    let cs = CString::new("ffi-string").unwrap();
    println!("into_string = {:?}", cs.into_string());

    println!("\n6. \".\".parse::<f64>() 从 Ok(0.0) 改为 Err（1.7 兼容性变化）");
    println!("parse 结果 = {:?}", ".".parse::<f64>());

    println!("\n7. sort_by_key");
    let mut v = [(3, "c"), (1, "a"), (2, "b")];
    v.sort_by_key(|t| t.0);
    println!("{:?}", v);
}

use std::hash::Hash;
