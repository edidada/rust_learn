// rustc 1.13.0 演示 —— ? 运算符稳定化 / try_borrow / assert_ne!
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;

fn main() {
    println!("rustc 1.13.0 演示");

    println!("\n1. ? 运算符稳定化（当年形态：try! 宏）");
    // 1.0~1.12 时代错误传播要写 try!(expr)；1.13 起直接写 expr?
    println!("parse_double(\"21\") = {:?}", parse_double("21"));
    println!("parse_double(\"2x1\") = {:?}", parse_double("2x1"));

    println!("\n2. RefCell::try_borrow / try_borrow_mut（1.13 稳定）");
    let cell = RefCell::new(1_i32);
    let ok = cell.try_borrow_mut();
    println!("try_borrow_mut 首次 = {:?}", ok.is_ok());
    let _held = cell.borrow();
    let conflict = cell.try_borrow_mut();
    println!("占用期间 try_borrow_mut 失败 = {}", conflict.is_err()); // 不 panic，返回 Err

    println!("\n3. assert_ne!（1.13 新增，此前只有 assert!）");
    let a = 1;
    let b = 2;
    assert_ne!(a, b, "a 不应等于 b");
    println!("assert_ne!(1, 2) 通过");

    println!("\n4. checked_abs / wrapping_abs / overflowing_abs");
    println!("(-5).checked_abs() = {:?}", (-5_i32).checked_abs());
    println!("i32::MIN.wrapping_abs() = {}", i32::MIN.wrapping_abs());
    println!("i32::MIN.overflowing_abs() = {:?}", i32::MIN.overflowing_abs());

    println!("\n5. DefaultHasher（SipHasher 1.13 起弃用后的替代）");
    let mut h = DefaultHasher::new();
    "seed".hash(&mut h);
    println!("DefaultHasher 哈希 = {}", h.finish());

    println!("\n6. 语句属性 / 类型位置宏（println 讲解节）");
    // 当年形态：属性稳定到语句上，如 #[allow(...)] let x = ...;
    // 宏可用在类型位置：type Alias = vec![0u8; 4] 之类由宏产出的类型（RFC 873）。
    #[allow(unused_variables)]
    let marked = 0_i32;
    println!("语句属性 #[allow(unused_variables)] let marked = 0; 已生效。");
}

// 1.13 前：Ok(try!(s.parse::<i32>()) * 2)；1.13 起用 ?
fn parse_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}
