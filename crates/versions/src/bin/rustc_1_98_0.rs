// rustc 1.98.0 演示 —— substr_range、strip_circumfix、Atomic::from_mut、algebraic_*、NumBuffer、from_utf16le/be、from_str_radix、derive
// 注意：本仓库运行环境即 rustc 1.98，全部可直接演示。
use core::fmt::NumBuffer;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::num::NonZeroU32;

fn main() {
    println!("rustc 1.98.0 演示");

    println!("\n1. str::substr_range / <[T]>::subslice_range（1.98 稳定）");
    // 给出子串/子切片在原数据中的字节区间 —— split 配合定位场景
    let data = "a, b, b, a";
    let positions: Vec<(usize, usize)> = data
        .split(", ")
        .map(|s| data.substr_range(s).unwrap())
        .map(|r| (r.start, r.end))
        .collect();
    println!("各子片段位置 = {positions:?}");
    let arr = [0u8, 5, 10, 0, 0, 5];
    // 注意：subslice_range 按"指针来自原切片"定位（不按值比较），返回 core::range::Range
    let parts: Vec<(usize, usize)> = arr
        .split(|t| *t == 0)
        .map(|p| {
            let r = arr.subslice_range(p).unwrap();
            (r.start, r.end)
        })
        .collect();
    println!("按指针定位的各段位置 = {parts:?}");

    println!("\n2. str / <[T]>::strip_circumfix（1.98 稳定：剥环绕对）");
    // 旧写法：strip_prefix 后再 strip_suffix 两步；strip_circumfix 一步并检查重叠
    println!("\"bar:hello:foo\" 剥 bar:/foo → {:?}", "bar:hello:foo".strip_circumfix("bar:", ":foo"));
    println!("\"(hello)\" 剥圆括号 → {:?}", "(hello)".strip_circumfix('(', ')'));
    println!("重叠情形 \"foo:bar:baz\" → {:?}", "foo:bar:baz".strip_circumfix("foo:bar:", ":bar:baz"));
    let v = [10u8, 50, 40, 30];
    println!("[10,50,40,30] 剥 [10]…[30] → {:?}", v.strip_circumfix(&[10], &[30]));

    println!("\n3. Atomic<T>::from_mut（1.98 稳定：&mut 引用直接变原子视图）");
    // AtomicBool = Atomic<bool>；from_mut 零拷贝把普通变量升级为原子操作对象
    let mut flag = false;
    let a = AtomicBool::from_mut(&mut flag);
    a.store(true, Ordering::SeqCst);
    println!("AtomicBool::from_mut 后 flag = {flag}");

    println!("\n4. Atomic<T>::from_mut_slice / get_mut_slice（1.98 稳定）");
    let mut nums = [0u32; 5];
    {
        let atomic_view: &mut [AtomicU32] = AtomicU32::from_mut_slice(&mut nums);
        for (i, slot) in atomic_view.iter().enumerate() {
            slot.store(i as u32, Ordering::Relaxed);
        }
        println!("atomic 写入后 nums（通过原子视图读） = {:?}",
            atomic_view.iter().map(|s| s.load(Ordering::Relaxed)).collect::<Vec<u32>>());
        // get_mut_slice：把原子切片视图转回普通可变切片（独占借用保证安全）
        let plain: &mut [u32] = AtomicU32::get_mut_slice(atomic_view);
        plain[0] = 100;
    }
    println!("get_mut_slice 改回非原子访问 nums[0] = {}", nums[0]);

    println!("\n5. {{fN}}::algebraic_*（1.98 稳定：允许代数化简的浮点运算）");
    // 与普通 +-*/ 的区别：允许编译器按代数规则重排（如去括号），结果可能因优化而与 IEEE 严格顺序不同
    let a = 1.0f32;
    let b = 2.0f32;
    println!("algebraic_add(1,2) = {}", a.algebraic_add(b));
    println!("algebraic_mul(1,2) = {}", a.algebraic_mul(b));
    println!("algebraic_sub(1,2) = {}", a.algebraic_sub(b));
    println!("algebraic_div(1,2) = {}", a.algebraic_div(b));

    println!("\n6. core::fmt::NumBuffer::format_into（1.98 稳定：无分配整数格式化）");
    // 旧写法 format! 走堆分配；NumBuffer 用栈缓冲一次声明复用，性能敏感路径友好
    let mut buf = NumBuffer::<u32>::new();
    let s = 1972u32.format_into(&mut buf);
    println!("1972.format_into(buf) = {s}");
    let mut sbuf = NumBuffer::<i32>::new();
    let s2 = (-1972i32).format_into(&mut sbuf);
    println!("(-1972).format_into(buf) = {s2}");

    println!("\n7. String::from_utf16le / from_utf16be（1.98 稳定）");
    // UTF-16 字节序显式化：le=小端 be=大端；lossy 版用 U+FFFD 替换无效对
    let le_bytes: [u8; 6] = [0x48, 0x00, 0x69, 0x00, 0x21, 0x00]; // "Hi!" 小端编码
    println!("from_utf16le = {:?}", String::from_utf16le(&le_bytes));
    println!("from_utf16le_lossy = {:?}", String::from_utf16le_lossy(&le_bytes));

    println!("\n8. NonZero::from_str_radix（1.98 稳定）");
    // 旧写法：先 parse 再 NonZero::new 包一层；一步解析并保证非零
    println!("NonZeroU32::from_str_radix(\"ff\", 16) = {:?}", NonZeroU32::from_str_radix("ff", 16));
    println!("NonZeroU32::from_str_radix(\"0\", 10) = {:?}", NonZeroU32::from_str_radix("0", 10)); // Err

    println!("\n9. {{std}}::derive 宏显式稳定（1.98，MSRV 记 1.96）");
    // #[derive] 既是内建属性也是宏；以前意外稳定过一次，1.98 明确把 {core,std}::derive 作为稳定路径
    println!("std::derive 宏存在：可用 use std::derive; 形式引用（教学记叙）");
    println!("其余兼容性：repr(transparent) trivial 字段更严；Vars 不再 Send/Sync；v0 后 transmute 大小检查更准");
}
