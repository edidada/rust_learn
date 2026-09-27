// rustc 1.9.0 演示 —— panic::catch_unwind / copy_from_slice / HashSet take
use std::collections::HashSet;
use std::panic;

fn main() {
    println!("rustc 1.9.0 演示");

    println!("\n1. std::panic::catch_unwind（当年形态：recover，1.9 改名稳定）");
    // 把 panic 转为 Result，用于 FFI 边界 / 测试框架等场景
    let r = panic::catch_unwind(|| {
        panic!("boom");
    });
    println!("catch_unwind 结果 = {}", r.is_err());
    let ok = panic::catch_unwind(|| 40 + 2);
    println!("正常路径 = {:?}", ok);

    println!("\n2. slice::copy_from_slice（等长 memcpy 语义）");
    let src = [1_u8, 2, 3, 4];
    let mut dst = [0_u8; 4];
    dst.copy_from_slice(&src);
    println!("dst = {:?}", dst);

    println!("\n3. str::is_char_boundary");
    let s = "héllo";
    println!("is_char_boundary(0) = {}, (1) = {}, (2) = {}", s.is_char_boundary(0), s.is_char_boundary(1), s.is_char_boundary(2));

    println!("\n4. HashSet::get / replace / take（1.9 稳定）");
    let mut set: HashSet<String> = ["a".to_string(), "b".to_string()].into();
    println!("get(\"a\") = {:?}", set.get("a"));
    let old = set.replace("b".to_string());
    println!("replace(\"b\") 旧值 = {:?}", old);
    let taken = set.take("a");
    println!("take(\"a\") = {:?}，剩余 {:?}", taken, set);

    println!("\n5. char::decode_utf16 与未配对代理");
    // 有效代理对 → 一个字符；孤立代理 → Err
    let pairs = [0xD83D_u16, 0xDE00_u16, 0xD800_u16];
    let decoded: Vec<Result<char, _>> = char::decode_utf16(pairs).collect();
    println!("decode_utf16 = {:?}", decoded.iter().map(|r| r.is_ok()).collect::<Vec<_>>());

    println!("\n6. fn item 类型零大小（1.9 语言规则）");
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    // 每个 fn 名都是独一无二的零大小类型；取其指针才有大小
    println!("size_of_val(&add_one) = {}", std::mem::size_of_val(&add_one));
    println!("size_of::<fn(i32) -> i32>()（函数指针） = {}", std::mem::size_of::<fn(i32) -> i32>());

    println!("\n7. From<[u8; 4]> for Ipv4Addr");
    let ip = std::net::Ipv4Addr::from([8, 8, 8, 8]);
    println!("[8,8,8,8] -> {}", ip);
}
