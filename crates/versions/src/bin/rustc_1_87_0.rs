// rustc 1.87.0 演示 —— RPITIT use<>（重点）、asm goto 讲解、extract_if、split_off、管道、is_multiple_of
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.87 稳定的 API 在 1.98 均可用。

// 1.87 重点：precise_capturing_in_traits —— trait 方法的 RPITIT 可写 use<…> 边界。
// 实测规则（本仓库 1.98 工具链）：
//  1) trait 声明里 Self 被隐式捕获，必须写进 use 列表：`use<Self, T>`；
//  2) impl 处 Self 只是别名（E0799），use 列表只接受"泛型参数名"，
//     不能写具体类型（Evens/u8 报 E0799），也不能写 {Self}（解析错误）。
//     因此 trait 方法形态的精确捕获在 impl 处写法受限于别名 —— 教学演示
//     采用 trait 声明对照 + 自由函数的 use<'_, T>（官方 keyword 文档示例形态）。
// use<> 的价值：显式控制返回的 opaque type 捕获哪些参数——
// 本迭代器不借用 &self 的生命周期，故匿名生命周期不被捕获。
trait Series<T> {
    fn series(&self, t: u32) -> impl Iterator<Item = u32> + use<Self, T>;
}

struct Evens;
impl<T> Series<T> for Evens {
    // impl 处 Self 是别名（E0799）。实测：impl 处不能写 Self，只能写
    // 该 impl 可见的泛型参数 T —— 与 trait 声明的 use<Self, T> 捕获集合
    // 语义等价（Self 在实现处的捕获通过 impl 类型自动满足，编译器校验
    // 捕获集合兼容性而非字面一致）：
    fn series(&self, t: u32) -> impl Iterator<Item = u32> + use<T> {
        (0..3).map(move |i| i * 2 + t) // 与 self 无借用关系
    }
}

// 自由函数形态的精确捕获（1.82 起稳定，1.87 起也可用于 trait 内）：
fn precise_pick<'a>(nums: &'a [i32]) -> impl Iterator<Item = i32> + use<'a> {
    nums.iter().copied().map(|x| x * 3)
}

fn main() {
    println!("rustc 1.87.0 演示");

    println!("\n1. RPITIT use<…> 精确捕获（1.87 重点代码）");
    let s = Evens;
    let got: Vec<u32> = <Evens as Series<u8>>::series(&s, 10).collect();
    println!("series(10) = {got:?}（trait 方法声明 impl Iterator + use<Self, T>）");
    let data = vec![1, 2, 3];
    let picked: Vec<i32> = precise_pick(&data).collect();
    println!("precise_pick = {picked:?}（自由函数 impl Iterator + use<'_>）");
    // 若迭代器需要借用 &self，则 use<'_> 之类要显式列生命周期——这正是精确捕获的价值：
    // 返回类型可以"不借 self"，从而让 trait 对象/多线程场景更自由。

    println!("\n2. asm goto（asm! 的 label 支持，1.87 稳定）—— 注释讲解");
    // asm_goto 允许内联汇编跳到 Rust 标签，形如：
    //   let out;
    //   unsafe { asm!("jmp {}", label /* 需目标平台支持 */, ...) }
    // 语法（示意，x86_64 才可运行，故本演示不内联执行）：
    //   unsafe { asm!("jmp {}", label { println!("跳转到了 Rust 标签"); }, in ... ) }
    // 平台差异：label 分支在 x86/x86_64/arm64 可用；其他平台可能不支持，因此这里只讲解。
    println!("（asm goto 语法稳定，但需平台支持，讲解见注释；运行输出由 println 模拟）");
    println!("模拟结果：asm 内 jcc 条件分支 → 跳转到 label 处继续执行 Rust 代码");

    println!("\n3. Vec::extract_if / LinkedList::extract_if（1.87 稳定）");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    // Vec::extract_if 在 1.87 采用带 range 的新签名：extract_if(range, predicate)
    let removed: Vec<i32> = v.extract_if(.., |x| *x % 2 == 0).collect();
    println!("extract_if(偶数) 移走 = {removed:?}, 剩余 = {v:?}");
    let mut l = std::collections::LinkedList::from([10, 15, 20]);
    let removed_l: Vec<i32> = l.extract_if(|x| *x < 16).collect();
    println!("LinkedList extract_if(<16) = {removed_l:?}, 剩余 = {l:?}");

    println!("\n4. <[_]>::split_off 系列（1.87 稳定，作用在 &mut 切片变量上）");
    // 形态：split_off 接收单侧区间，并原地缩短切片变量（self: &mut &Self）：
    let mut s: &[i32] = &[1, 2, 3, 4];
    let tail = s.split_off(2..);
    println!("split_off(2..) 前半 = {s:?}, 取走后半 = {tail:?}");
    let mut s2: &mut [i32] = &mut [1, 2, 3, 4];
    let first = s2.split_off_first_mut(); // Option<&mut T>
    let rest = s2; // 切片变量被缩短为 [2,3,4]
    println!("split_off_first_mut = {first:?}, 剩余 = {rest:?}");

    println!("\n5. is_multiple_of / unbounded_shl / cast_signed（1.87 稳定）");
    println!("12u32.is_multiple_of(4) = {}", 12u32.is_multiple_of(4));
    println!("1u32.unbounded_shl(40) = {}（按语义回绕，不 panic/不掩码位数）", 1u32.unbounded_shl(40));
    println!("i32::from(3u32).cast_signed 用法见下：");
    println!("(3u32).cast_signed() = {}", 3u32.cast_signed());

    println!("\n6. io::pipe —— 匿名管道（1.87 稳定）");
    let (mut reader, mut writer) = std::io::pipe().unwrap();
    writer.write_all(b"through-pipe").unwrap();
    drop(writer); // 关闭写端，读端才能读到 EOF
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    println!("pipe 传输 = {buf}");
    use std::io::{Read, Write};

    println!("\n7. OsStr::display / TryFrom<Vec<u8>> for String（1.87 稳定）");
    let os = std::ffi::OsString::from("os-str-87");
    println!("OsString::display() = {}", os.display()); // 不再要 to_string_lossy
    let s = String::try_from(b"utf8-ok".to_vec());
    println!("String::try_from(vec![u8]) = {:?}", s);
}
