// rustc 1.90.0 演示 —— u*_sub_signed、CStr/CString 比较、浮点 const 化、reverse const 化、IntErrorKind
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::num::IntErrorKind;

fn main() {
    println!("rustc 1.90.0 演示");

    println!("\n1. uN::*_sub_signed（1.90 稳定：无符号减有符号数族）");
    // 旧写法要 wrapping_sub(或 checked_sub) 先做有符号数取负/换算；1.90 直接减一个有符号数
    let a: u32 = 3;
    println!("3u32.checked_sub_signed(-5)   = {:?}", a.checked_sub_signed(-5)); // Some(8)
    println!("3u32.saturating_sub_signed(9) = {}", a.saturating_sub_signed(9)); // 0
    println!("3u32.wrapping_sub_signed(-1)  = {}", a.wrapping_sub_signed(-1)); // 4
    println!("3u32.overflowing_sub_signed(-5)= {:?}", a.overflowing_sub_signed(-5)); // (8, false)
    println!("88u8.wrapping_sub_signed(-44) = {}", 88u8.wrapping_sub_signed(-44)); // 132

    println!("\n2. CStr / CString / Cow<CStr> 互相比较（1.90 稳定）");
    let cs: &CStr = c"hello"; // C 字符串字面量
    let cstring: CString = CString::new("hello").unwrap();
    let cow: Cow<CStr> = Cow::Borrowed(cs);
    println!("&CStr == &CStr      -> {}", cs == cstring.as_c_str());
    println!("CString == &CStr    -> {}", cstring == cs); // PartialEq<&CStr> for CString
    println!("Cow<CStr> == CStr   -> {}", cow == *cs.as_ref());
    println!("Cow<Owned> == &CStr -> {}", Cow::Owned(cstring.clone()) == cs);

    println!("\n3. 浮点 floor/ceil/trunc/fract/round/round_ties_even 进 const（1.90）");
    // 旧版本这些在 const 上下文是 error[E0658]；1.90 全部 const 稳定
    const X: f64 = -2.75;
    const FLOOR: f64 = X.floor();
    const CEIL: f64 = X.ceil();
    const TRUNC: f64 = X.trunc();
    const FRACT: f64 = X.fract();
    const ROUND: f64 = X.round();
    const RTE: f64 = 2.5f64.round_ties_even(); // 银行家舍入 → 2.0
    println!("floor={FLOOR} ceil={CEIL} trunc={TRUNC} fract={FRACT} round={ROUND}");
    println!("round_ties_even(2.5) = {RTE}");

    println!("\n4. <[T]>::reverse const 化（1.90）");
    const REV: [u8; 4] = {
        let mut a = [1, 2, 3, 4];
        a.reverse(); // 现在 const 可用
        a
    };
    println!("const [1,2,3,4].reverse() = {REV:?}");

    println!("\n5. IntErrorKind 实现 Copy/Hash（1.90）");
    fn kind_of(s: &str) -> IntErrorKind {
        s.parse::<i32>().unwrap_err().kind().clone() // kind() 返回 &IntErrorKind；Copy 后可按值
    }
    let k1 = kind_of("9999999999999"); // PosOverflow
    let k2 = k1; // Copy
    println!("k1 == k2（同为 PosOverflow）？ {}", k1 == k2);

    println!("\n6. 兼容性注记（记叙）");
    println!("- x86_64-unknown-linux-gnu 默认使用 lld 链接");
    println!("- Fuse::default() 现按文档构造内部迭代器 I::default()（以前总是空终态）");
    println!("- UnixStream 请求 MSG_NOSIGNAL：写端对端关闭返回错误而非 SIGPIPE 信号");
}
