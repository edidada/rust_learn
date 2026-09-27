// rustc 1.47.0 演示 —— 数组任意长度 trait / is_empty / Result::as_deref / Vec::leak / offset_from / TAU / const int 方法
use std::ops::{Range, RangeFull};

// 1.47.0 亮点：std/core 的 trait 实现不再限制数组长度 ≤ 32，任意 [T; N] 都满足。
// 此前写 64 元素数组会因 trait 只实现到 32 而编不过；1.47 起 OK。
const BIG_TABLE: [[&str; 8]; 40] = [[""; 8]; 40]; // 40 行 × 8 列，作为数组可直接 Debug/Clone 等

fn main() {
    println!("rustc 1.47.0 演示");

    // 1.47.0：任意长度数组实现 std trait（Debug/PartialEq/Copy……等等）。
    println!("\n1. 数组任意长度实现 trait");
    let mut arr64 = [0i64; 64]; // 64 元素数组；1.46 及之前 Debug 等实现止步 32
    arr64[0] = 7;
    println!("64 元素数组可直接 Debug：[{:?}, ...]", arr64[..1].first());
    let arr33 = [3u8; 33]; // 33 > 32，此前编译不过的典型尺寸
    println!("33 元素数组 len = {}，Clone/Debug 均可用", arr33.len());
    println!("40×8 嵌套常量 BIG_TABLE 占用 {} 个元素", BIG_TABLE.len() * BIG_TABLE[0].len());

    // 1.47.0 稳定：Range::is_empty、RangeInclusive::is_empty。
    println!("\n2. Range::is_empty / RangeInclusive::is_empty");
    println!("(0..0).is_empty()       = {}", (0..0).is_empty());
    println!("(0..5).is_empty()       = {}", (0..5).is_empty());
    println!("(0..=3).is_empty()      = {}", (0..=3).is_empty());
    println!("(3..=3).is_empty()      = {}（含端点时非空）", (3..=3).is_empty());

    // 1.47.0 稳定：Result::as_deref / as_deref_mut。
    println!("\n3. Result::as_deref / Result::as_deref_mut");
    let r: Result<String, ()> = Ok("hello".to_string());
    let s: Result<&str, _> = r.as_deref(); // Ok(String) → Ok(&str)
    println!("Ok(String).as_deref() = {:?}", s.map(|v| v.to_uppercase()));
    let mut buf: Result<Vec<u8>, ()> = Ok(vec![1, 2, 3]);
    if let Ok(v) = buf.as_deref_mut() {
        v[0] = 99; // 拿到 &mut [u8] 直接改
        println!("as_deref_mut 改首元素 = {:?}", v);
    }

    // 1.47.0 稳定：Vec::leak —— 生命周期内丢弃容器，泄漏出 &'a mut [T]。
    println!("\n4. Vec::leak");
    let leaked: &'static mut [char] = vec!['r', 'u', 's', 't'].leak();
    leaked[0] = 'R';
    println!("leak 后可直接索引且生命周期是 'static：{}（len = {}）", leaked[0], leaked.len());

    // 1.47.0 稳定：pointer::offset_from —— 同一分配内两裸指针的有符号距离。
    println!("\n5. pointer::offset_from");
    let data = [10i32, 20, 30, 40, 50];
    let first = &data[0] as *const i32;
    let third = &data[2] as *const i32;
    let diff: isize = unsafe { third.offset_from(first) };
    println!("third.offset_from(first) = {}（两指针相差的元素数）", diff);
    let neg: isize = unsafe { first.offset_from(third) };
    println!("反向求差 = {}（有符号，方向正确）", neg);

    // 1.47.0 稳定：f32::TAU / f64::TAU（τ = 2π）。
    println!("\n6. f32::TAU / f64::TAU");
    println!("f64::TAU = {}", std::f64::consts::TAU);
    println!("f32::TAU = {}", std::f32::consts::TAU);
    println!("π 两倍验证：2 × π = {}", 2.0 * std::f64::consts::PI);

    // 1.47.0：大量整数方法成为 const fn —— 可直接在 const 上下文算。
    println!("\n7. const fn：checked/saturating、NonZero::new、signum、is_ascii*");
    const NZ: std::num::NonZeroI32 = std::num::NonZeroI32::new(42).unwrap(); // NonZero::new 是 const
    const CHECKED: Option<i32> = 100i32.checked_add(-50); // 1.47 起可在 const 求值
    const SAT: i32 = 100i32.saturating_mul(3);
    const SIGNUM: i32 = (-7i32).signum();
    const IS_DIGIT: bool = '7'.is_ascii_digit();
    println!("const NonZeroI32 = {}", NZ.get());
    println!("const checked_add = {:?}", CHECKED);
    println!("const saturating_mul(100×3) = {}", SAT);
    println!("const (-7).signum() = {}", SIGNUM);
    println!("const '7'.is_ascii_digit() = {}", IS_DIGIT);

    // 1.47.0 其他库变化：CStr 索引、RangeFull/Range Default、panic::Location 派生。
    println!("\n8. CStr 索引 / RangeFull & Range Default / Location traits");
    let cstr = std::ffi::CStr::from_bytes_with_nul(b"abc\0").unwrap();
    let tail = &cstr[1..]; // CStr: Index<RangeFrom<usize>>，取尾字节
    println!("CStr 索引 cstr[1..] 含 {} 字节（去掉首字符）", tail.to_bytes().len());
    let full: RangeFull = RangeFull::default(); // RangeFull 实现 Default
    let one: Range<i32> = Range::default(); // 0..0
    println!("RangeFull::default 存在；Range::default = {:?}", (one.start, one.end));
    let loc = std::panic::Location::caller(); // Location 有 Copy/Ord 等
    let loc_copy = loc; // Copy 语义
    println!("Location::caller = {:?}（Copy 后相同 = {:?}）", loc, loc_copy);
}
