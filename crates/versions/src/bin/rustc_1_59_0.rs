// rustc 1.59.0 演示 —— destructuring assignment、const 泛型默认值、iter::zip
fn main() {
    println!("rustc 1.59.0 演示");

    // ============================================================
    println!("\n1. const 泛型参数默认值与排序放宽");
    // 1.59 稳定：<const N: usize = 3>；类型与 const 参数可交错排列。
    struct Buf<const N: usize = 3> {
        data: [u8; N],
    }
    impl<const N: usize> Buf<N> {
        fn len(&self) -> usize {
            N
        }
    }
    // 用默认 N：Buf
    let b = Buf { data: [1, 2, 3] };
    println!("  默认 N 的 Buf.len() = {}", b.len());
    // 显式 N
    let b5: Buf<5> = Buf { data: [0; 5] };
    println!("  Buf<5>.len() = {}", b5.len());

    // ============================================================
    println!("\n2. destructuring assignment 模式赋值");
    // 1.59 稳定：左侧可用 (全局/struct/slice) 模式解构直接赋值。
    // 对比旧行为：只能整个 let，无法在循环里就地解构已有变量。
    let (mut a, mut b);
    (a, b) = (1, 2);
    println!("  (a, b) = (1, 2) -> a={} b={}", a, b);
    let mut p = (9, 8);
    (p.0, p.1) = (p.1, p.0);
    println!("  元组两元素交换 -> {:?}", p);
    let list = [10u8, 20];
    let mut x3; let mut x4;
    [x3, x4] = list;
    println!("  [x,y] = [10,20] -> x={} y={}", x3, x4);
    // 注意：destructuring assignment 只支持普通 `=`，不支持 += 等复合运算符；
    // 逐元素加 1 要写成对偶式的普通赋值：
    let (mut m, mut n) = (3, 4);
    (m, n) = (m + 1, n + 1);
    println!("  (m, n) = (m+1, n+1) -> m={} n={}", m, n);
    // 注意：destructuring assignment 不引入新绑定，左侧变量须先前已声明。

    // ============================================================
    println!("\n3. iter::zip 自由函数");
    use std::iter::zip;
    // 1.59 稳定：zip(a, b) 自由函数；旧写法 a.into_iter().zip(b)。
    let z: Vec<(u8, u8)> = zip([1, 2, 3], [9, 8]).collect();
    println!("  zip([1,2,3], [9,8]) = {:?}", z); // 短的截断

    // ============================================================
    println!("\n4. TryFrom 扩展：&mut [T] -> [T; N]、char -> u8");
    let mut buf = [5u8, 6, 7];
    let arr: [u8; 3] = <[u8; 3]>::try_from(&mut buf[..]).unwrap();
    println!("  <[u8;3]>::try_from(&mut slice) = {:?}", arr);
    println!("  u8::try_from('a') = {:?}", u8::try_from('a')); // Some(97)

    // ============================================================
    println!("\n5. Result::copied/cloned 与 available_parallelism");
    let r: &[u8] = &[10];
    let got: Option<u8> = r.first().copied();
    println!("  first().copied() = {:?}", got);
    println!(
        "  available_parallelism() = {:?} 个逻辑核可用",
        std::thread::available_parallelism().map(|n| n.get())
    );

    // ============================================================
    println!("\n6. NonZero*::is_power_of_two");
    use std::num::NonZeroU32;
    let nz = NonZeroU32::new(8).unwrap();
    println!("  8.is_power_of_two() = {}", nz.is_power_of_two());
    let nz2 = NonZeroU32::new(7).unwrap();
    println!("  7.is_power_of_two() = {}", nz2.is_power_of_two());

    // ============================================================
    println!("\n7. 兼容性提醒");
    println!("  - unreachable!() 对齐 2021 格式化宏行为（不再直接显示非常量参数）。");
    println!("  - zip 允许底层迭代器多 advance（自写 unsafe 代码需注意）。");
    println!("  - 空切片 split_inclusive() 输出空；temp_dir 走 GetTempPath2。");
}
