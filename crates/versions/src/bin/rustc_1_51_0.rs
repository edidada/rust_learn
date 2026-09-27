// rustc 1.51.0 演示 —— const generics MVP、addr_of!、split_inclusive
fn main() {
    println!("rustc 1.51.0 演示");

    // ============================================================
    println!("\n1. const generics MVP：struct Arr<T, const N: usize>");
    // 1.51 稳定 const 泛型（MVP）：类型参数之外可再按常量参数化。
    // 目前仅允许整型/bool/char 的常量参数（不支持 &str、自定义类型等）。
    // 对比旧行为：以前要么 [T; N] 硬编码长度，要么用运行时 Vec/宏生成。
    struct Arr<T, const N: usize> {
        inner: [T; N],
    }

    impl<T, const N: usize> Arr<T, N> {
        // const N 在 impl 块内直接可用，还能参与条件分支
        fn len(&self) -> usize {
            N
        }
    }

    impl<T: Copy + Default, const N: usize> Arr<T, N> {
        fn first_or_default(&self) -> T {
            if N == 0 {
                T::default()
            } else {
                self.inner[0]
            }
        }
    }

    let a = Arr { inner: [1u8, 2, 3] };
    let b: Arr<i32, 0> = Arr { inner: [] };
    println!("  Arr<u8,3>.len() = {}", a.len());
    println!("  Arr<i32,0>.first_or_default() = {:?}", b.first_or_default());

    // ============================================================
    println!("\n2. const 泛型函数：一套代码适配所有 N");
    // 函数也能按 <const N: usize> 泛型化；调用时从字面量/数组推断 N。
    fn sum<const N: usize>(arr: [u32; N]) -> u32 {
        // arr.iter().sum::<u32>()
        let mut s = 0u32;
        for v in arr.iter() {
            s += v;
        }
        s
    }
    println!("  sum([1,2,3]) = {}", sum([1, 2, 3]));
    println!("  sum([10; 5]) = {}", sum([10; 5]));
    // 显式指定 const 参数：fn_name::<3>(...) 形式（此处推断也够用）。

    // ============================================================
    println!("\n3. ptr::addr_of! / addr_of_mut!");
    // 1.51 稳定：直接生成裸指针，不创建中间 &T/&mut T 引用。
    // 用途：绕开对未对齐/未初始化内存创建引用是 UB 的坑。
    #[repr(packed)]
    struct Unaligned {
        lo: u8,
        hi: u16, // 位于偏移 1，未 2 字节对齐：直接 &local.hi 属于对齐 UB
    }
    let mut local = Unaligned { lo: 1, hi: 0x0203 };
    // addr_of!：不创建引用，安全取得未对齐字段的裸指针
    let hi_ptr = std::ptr::addr_of!(local.hi);
    println!("  addr_of!(local.hi) 读出 = {}", unsafe { *hi_ptr });
    // addr_of_mut!：同样不建引用即可写
    let hi_mut = std::ptr::addr_of_mut!(local.hi);
    unsafe { *hi_mut = 0x0405 };
    println!("  addr_of_mut! 写后再读 = {}", unsafe { *hi_ptr });

    // ============================================================
    println!("\n4. slice::split_inclusive 与 strip_prefix/strip_suffix");
    // 1.51 稳定：split_inclusive 时分隔符留在下一段开头（面向行解析常见语义）。
    let lines = "a\nb\nc";
    for seg in lines.split_inclusive('\n') {
        println!("  split_inclusive 段: {:?}", seg);
    }
    // strip_prefix / strip_suffix：若匹配则返回去掉后的子切片。
    let s = "prefix/data";
    println!("  strip_prefix('prefix/') -> {:?}", s.strip_prefix("prefix/"));

    // ============================================================
    println!("\n5. VecDeque::range / Peekable::next_if / array::IntoIter");
    use std::collections::VecDeque;
    use std::iter::Peekable;
    let dq: VecDeque<i32> = [10, 20, 30, 40].into_iter().collect();
    // range(2..4)：返回下标范围的双端迭代
    for v in dq.range(2..4) {
        println!("  VecDeque::range(2..4) 取 {}", v);
    }
    // Peekable::next_if：若下一元素满足谓词则取走，否则不消耗
    let mut it = Peekable::new(vec![5, 1, 2].into_iter());
    println!("  next_if(>=5) -> {:?}", it.next_if(|&x| x >= 5));
    println!("  next_if(>=5) -> {:?}", it.next_if(|&x| x >= 5)); // None，1 未被消耗
    // array::IntoIter（1.51 稳定）：数组的按值迭代器
    let ai = std::array::IntoIter::new([1u8, 2, 3]);
    for (i, v) in ai.enumerate() {
        println!("  array::IntoIter[{}] = {}", i, v);
    }

    // ============================================================
    println!("\n6. 兼容性提醒");
    println!("  - atomic::spin_loop_hint 弃用，改用 hint::spin_loop（println 讲解）");
}
