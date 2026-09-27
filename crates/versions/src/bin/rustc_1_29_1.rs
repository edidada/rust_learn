// rustc 1.29.1 演示 —— 安全补丁：str::repeat 溢出改确定性 panic
// 该版本 introduces:
// str::repeat 此前存在整数溢出导致的越界写入；
// 1.29.1 起溢出时 deterministically panic（不再越界写内存）。

fn main() {
    println!("rustc 1.29.1 演示");
    println!("\n1. 安全修复（讲解）");
    println!("str::repeat 的整数溢出越界写 → 现在溢出时确定性 panic");
    println!("\n2. str::repeat 正常用法");
    let s = "ab".repeat(3);
    println!("\"ab\".repeat(3) = {}", s);
    // 注意：不要演示 repeat(usize::MAX)，那会真的 panic/溢出。
    println!("repeat(usize::MAX) 会 panic（演示略）");
}
