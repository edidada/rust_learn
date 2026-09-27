// rustc 1.79.0 演示 —— inline const `const {}`、associated type bounds、path::absolute、const API
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.79 稳定的 API 在 1.98 均可用。
use std::path::Path;

// 关联类型界（RFC 2289，1.79 稳定）：`T: Iterator<Item: Clone>`
// 旧写法要写两个 where 子句：T: Iterator, <T as Iterator>::Item: Clone
fn copies<T>(iter: T) -> Vec<<T as Iterator>::Item>
where
    T: Iterator<Item: Clone>,
{
    iter.map(|x| x.clone()).collect()
}

fn main() {
    println!("rustc 1.79.0 演示");

    println!("\n1. inline const 表达式：`const {{ … }}`（1.79 重点）");
    // inline const 保证整段代码在编译期求值，即使里面有"运行期看上去会出错"的语义。
    // 典型价值：浮点 NaN 的常量传入、编译期断言、以及要求 const 的场景内做计算。
    let mode: usize = const { 5 % 3 };
    println!("const {{ 5 % 3 }} = {mode}（编译期求值）");
    // 与运行期等价表达式对比：结果相同，但 const {} 版本被编译为编译期常量。
    let runtime = 5_usize % 3;
    assert_eq!(mode, runtime);
    // 编译期把浮点特殊值变成位模式常量（旧时代需要关联常量绕路）：
    const BITS: [u8; 4] = (const { f32::NAN.to_bits() }).to_be_bytes();
    println!("const {{ f32::NAN.to_bits() }} 的大端字节 = {BITS:?}");
    // inline const 引用外部变量？不行——const 块只能用 const 环境的值，这正是其语义。

    println!("\n2. associated type bounds：`T: Iterator<Item: Clone>`");
    let v = copies(vec![1, 2, 3, 4].into_iter());
    println!("copies(1..=4) = {v:?}");
    let s = copies(String::from("ab").chars().collect::<Vec<char>>().into_iter());
    println!("copies(chars) = {s:?}");

    println!("\n3. path::absolute（1.79 稳定）");
    // 旧时代拿绝对路径要手动 join(current_dir)；现在一步得到绝对路径（不访问文件系统做规范化）。
    let abs = std::path::absolute("a/../b.txt").unwrap();
    println!("absolute(\"a/../b.txt\") = {}", abs.display());
    println!("是否绝对路径：{}", Path::new(&abs).is_absolute());

    println!("\n4. 1.79 稳定的 const 上下文 API 与新稳定 API");
    // panic::Location::caller 在 1.79 起可用于 const；运行期用法不变。
    let loc = std::panic::Location::caller();
    println!("Location::caller() 文件 = {}, 行 = {}", loc.file(), loc.line());
    // str::Utf8Chunks（1.79 稳定）：按 UTF-8 合法/不合法块切分字节串，检查局部损坏的数据。
    // 注意是 <[u8]>::utf8_chunks，作用于字节切片，而不是 Cow<str>。
    let bad = [b'a', 0xff, b'b'];
    for chunk in bad.utf8_chunks() {
        println!("utf8_chunk valid = {:?}, invalid = {:?}", chunk.valid(), chunk.invalid());
    }
}
