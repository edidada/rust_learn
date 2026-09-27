// rustc 1.16.0 演示 —— repeat/replacen / Duration::checked_* / cargo check
use std::time::Duration;

fn main() {
    println!("rustc 1.16.0 演示");

    println!("\n1. str::repeat / replacen（1.16 稳定）");
    println!("\"ab\".repeat(3) = {:?}", "ab".repeat(3));
    println!("\"aXaXa\".replacen(\"a\", \"-\", 2) = {:?}", "aXaXa".replacen("a", "-", 2));

    println!("\n2. Duration::checked_add / checked_mul（1.16 稳定）");
    let d = Duration::from_secs(5);
    println!("5s.checked_add(2s) = {:?}", d.checked_add(Duration::from_secs(2)));
    println!("MAX.checked_add(1ns) = {:?}", Duration::MAX.checked_add(Duration::from_nanos(1)));
    println!("1s.checked_mul(3) = {:?}", Duration::from_secs(1).checked_mul(3));

    println!("\n3. VecDeque::truncate / resize（1.16 稳定）");
    let mut dq: std::collections::VecDeque<i32> = (1..=6).collect();
    dq.truncate(3);
    println!("truncate(3) = {:?}", dq);
    dq.resize(5, 9);
    println!("resize(5, 9) = {:?}", dq);

    println!("\n4. Result::unwrap_or_default（1.16 稳定）");
    let bad: Result<i32, &str> = Err("nope");
    println!("unwrap_or_default = {}（Err 则给 i32::default()）", bad.unwrap_or_default());

    println!("\n5. Vec::dedup_by_key（1.16 稳定）");
    let mut v = vec![1, 1, 2, 2, 3];
    v.dedup_by_key(|x| *x);
    println!("{:?}", v);

    println!("\n6. String::insert_str / split_off（1.16 稳定）");
    let mut s = String::from("world");
    s.insert_str(0, "hello ");
    let tail = s.split_off(6);
    println!("s = {:?}, split_off 尾部 = {:?}", s, tail);

    println!("\n7. IpAddr::is_ipv4 / is_ipv6（1.16 稳定）");
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let v4 = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
    println!("v4.is_ipv4 = {}, v6.is_ipv6 = {}", v4.is_ipv4(), v6.is_ipv6());

    println!("\n8. cargo check（println 讲解节）");
    // 当年形态：1.16 推出 cargo check，只跑类型检查不链接产物，
    // 大幅加快"只想验证能否编译"的迭代速度。
    println!("cargo check：快速类型检查（不生成二进制）。");
}
