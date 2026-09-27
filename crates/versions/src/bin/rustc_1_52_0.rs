// rustc 1.52.0 演示 —— split_once、partition_point、char API、OsString 扩展
fn main() {
    println!("rustc 1.52.0 演示");

    // ============================================================
    println!("\n1. str::split_once / rsplit_once");
    // 1.52 稳定：一次性把字符串在第一个/最后一个分隔符处切成 (前, 后)。
    // 对比旧行为：以前要 splitn(2, sep) 再收集，还要手动处理 None。
    let kv = "key=value=extra";
    println!("  split_once('=')    -> {:?}", kv.split_once('='));
    println!("  rsplit_once('=')   -> {:?}", kv.rsplit_once('='));

    // ============================================================
    println!("\n2. slice::partition_point —— 二分找分界下标");
    // 1.52 稳定：假设切片已按谓词排好（false..true），返回第一个 true 的下标。
    // 对比旧写法：手写二分查找，容易 off-by-one。
    let sorted = [1, 2, 4, 8, 16];
    let p = sorted.partition_point(|&x| x < 8);
    println!("  partition_point(x<8) = {}", p); // 3
    // 也可用于“在有序数组里找插入位置”。

    // ============================================================
    println!("\n3. char::from_digit / char::MAX / Arguments::as_str");
    use std::fmt::Arguments;
    // from_digit：数字 -> char（进制 2..=36）
    println!("  char::from_digit(7, 10) -> {:?}", char::from_digit(7, 10));
    println!("  char::MAX = {:?}", char::MAX);
    // Arguments::as_str：仅当参数就是一个字面格式串时才返回 Some
    let args = format_args!("just a literal");
    println!("  Arguments::as_str -> {:?}", args.as_str());
    let args2 = format_args!("x={}", 1);
    println!("  Arguments::as_str(带参数) -> {:?}", args2.as_str()); // None

    // ============================================================
    println!("\n4. OsString 实现 Extend / FromIterator");
    use std::ffi::OsString;
    // 拼接路径/字节串更顺手：不保证 UTF-8 也能收
    let mut os = OsString::from("C:\\");
    os.extend(["Users", "\\me"].iter());
    println!("  OsString Extend -> {:?}", os);
    let joined: OsString = ["a", "b"].iter().collect();
    println!("  OsString FromIterator -> {:?}", joined);

    // ============================================================
    println!("\n5. 整数除法/取余 const 化");
    const Q: i32 = 17 / 5;
    const R: i32 = 17 % 5;
    println!("  const 17/5 = {}, 17%5 = {}", Q, R);
    // 对比旧行为：1.51 起不再提升“可能失败”的运算；1.52 明确除/余全部 const 可用。
    println!("  cmp::Reverse 加 #[repr(transparent)]；Arc<impl Error> 实现 Error");
}
