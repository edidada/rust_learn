// rustc 1.44.0 演示 —— const vec![] / Vec::from([T;N]) / PathBuf 容量 / to_int_unchecked / Layout 组合
use std::alloc::Layout;
use std::path::PathBuf;

fn main() {
    println!("rustc 1.44.0 演示");
    // 1.44.0 引入：vec![] 特化为 Vec::new()，因此可在 const 上下文使用（1.44 前 vec![] 是运行期构造）。
    println!("\n1. vec![] 用于 const");
    const EMPTY: Vec<u8> = vec![]; // 之前此行编译错误；1.44 起合法
    println!("const EMPTY: Vec<u8> 长度 = {}", EMPTY.len());
    let runtime_v = vec![1, 2, 3];
    println!("运行期 vec![1,2,3] 照旧 = {:?}", runtime_v);

    // 1.44.0 引入：Vec<T>: From<[T; N]>（N ≤ 32）。
    println!("\n2. Vec::from([T; N])");
    let arr = ["a", "b", "c"];
    let v = Vec::from(arr); // 之前需要 .to_vec()（依赖所有权数组）
    println!("Vec::from([...]) = {:?}", v);

    // 1.44.0 稳定：PathBuf 容量系列 API。
    println!("\n3. PathBuf 容量 API");
    let mut p = PathBuf::with_capacity(64); // 1.44 稳定
    p.push("some"); // 内部"Oh-So-Small"字符串路径缓冲
    p.push("longer-path");
    println!(" PathBuf 内容        = {:?}", p);
    println!("capacity()          = {}", p.capacity()); // 1.44 稳定
    p.shrink_to_fit(); // 1.44 稳定
    println!("shrink_to_fit 后 capacity = {}", p.capacity());
    p.clear(); // 1.44 稳定
    println!("clear() 后          = {:?}（仍保留容量）", p);

    // 1.44.0 稳定 f32/f64::to_int_unchecked。
    // 打印两种转换语义：`as` 截断（1.45 起还将饱和），to_int_unchecked 要求调用方保证在范围内。
    println!("\n4. to_int_unchecked 对照（重点：1.45 后 as 是饱和转换）");
    let f = 3.7f64;
    println!("3.7f64 as u8        = {}（旧写法；1.45 起越界则拦顶）", f as u8);
    let in_range: f64 = 42.9;
    let n: i32 = unsafe { in_range.to_int_unchecked() }; // 调用方保证不越界、非小数部分舍入向零
    println!("42.9.to_int_unchecked::<i32>() = {}（安全前提：值必须在目标范围）", n);

    // 1.44.0 稳定：Layout::{align_to, pad_to_align, extend, array}。
    println!("\n5. Layout 组合 API");
    let l1 = Layout::new::<u8>(); // size 1, align 1
    let l2 = Layout::new::<u32>(); // size 4, align 4
    let padded = l1.pad_to_align(); // 补齐到对齐 → size 4
    println!("u8 Layout pad_to_align => {{ s: {} }}", padded.size());
    let (combo, offset) = l1.extend(l2).unwrap(); // 串联两布局，offset 是第二个的起始
    println!("extend(u8,u32)：总体 size = {}, u32 前偏移 = {}", combo.size(), offset);
    let arr_l = Layout::new::<u32>().array(7).unwrap(); // 7 个 u32 连续布局
    println!("array(7 of u32) => size = {}", arr_l.size());
    let aligned = l1.align_to(16).unwrap();
    println!("align_to(16) => align = {}", aligned.align()); // 最高 16
}
