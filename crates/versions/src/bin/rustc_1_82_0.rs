// rustc 1.82.0 演示 —— &raw const/mut、unsafe extern、use<> 精确捕获、offset_of! 嵌套、const 浮点
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.82 稳定的 API 在 1.98 均可用。
use std::mem::offset_of;

// 1.82 稳定 unsafe extern 块（RFC 3484）：extern 块本身标 unsafe。
// edition 2024 里这是唯一合法写法；旧写法 `extern "C" { fn ... }` 在 2024 已拒绝。
// 这里声明 MSVC CRT 就有的 abs，无需 unsafe 函数体。
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

// 1.82 稳定 use<…> 精确捕获（RFC 3617）：返回位置 impl Trait 显式列出捕获的参数。
// 旧语义：impl Trait 默认捕获作用域内所有生命周期参数（edition 2021）；
// 新语义：只有 use<> 里列出的被捕获，"借用外部"的隐式捕获被关闭。
// use<> 只在 edition 2024 与（1.82 起）显式写出的场景可用。
fn pick(nums: &[i32]) -> impl Iterator<Item = i32> + use<'_> {
    nums.iter().copied().map(|x| x * 2)
}

// 嵌套字段 offset：演示 struct 布局
struct Inner {
    a: u32,
    b: u8,
}
struct Outer {
    head: u8,
    inner: Inner,
}

// const fn 中的浮点运算（1.82 稳定）：此前 const fn 里不能做浮点算术。
const fn circle_area(r: f64) -> f64 {
    3.14159 * r * r
}
const AREA_2: f64 = circle_area(2.0);

fn main() {
    println!("rustc 1.82.0 演示");

    println!("\n1. &raw const / &raw mut —— 不用 unsafe 取裸指针（1.82 重点）");
    let mut x = 42i32;
    // 旧写法：let p = &x as *const i32;（易和"指针转型"混淆）
    // 新写法：&raw const / &raw mut —— 直接"造指针"，不解引用、无 unsafe。
    let p: *const i32 = &raw const x;
    let pm: *mut i32 = &raw mut x;
    unsafe {
        *pm += 1; // 解引用仍是 unsafe（造指针免费，用指针收费）
    }
    println!("*p = {}，经 &raw mut 改后 x = {}", unsafe { *p }, x);
    // 与 addr_of! 的关系：&raw const x 等价于 addr_of!(x)，但不需要宏、可安全用于 static。

    println!("\n2. unsafe extern 块（edition 2024 强制）");
    let n = unsafe { abs(-5) }; // 调用外部 C 函数
    println!("abs(-5) = {n}（unsafe extern 块中声明）");

    println!("\n3. use<…> 精确捕获（1.82 稳定）");
    let v: Vec<i32> = pick(&[1, 2, 3]).collect();
    println!("pick(&[1,2,3]) = {v:?}（impl Iterator + use<'_>）");

    println!("\n4. offset_of! 嵌套字段（1.82 稳定）");
    // 旧写法只能 offset_of!(Inner, b)；1.82 起可以点穿多层：
    println!("offset_of!(Outer, inner.b) = {}", offset_of!(Outer, inner.b));
    println!("offset_of!(Outer, inner.a) = {}", offset_of!(Outer, inner.a));
    let o = Outer { head: 1, inner: Inner { a: 2, b: 3 } };
    println!("head 字段值（演示用）= {}", o.head);

    println!("\n5. const fn 中的浮点运算（1.82 稳定）");
    println!("const AREA_2 = {AREA_2}（编译期浮点乘法）");

    println!("\n6. is_sorted / is_none_or / repeat_n（1.82 稳定 API）");
    let s1 = [1, 2, 2, 3];
    let s2 = [3, 1, 2];
    println!("[1,2,2,3].is_sorted() = {}; [3,1,2].is_sorted() = {}", s1.is_sorted(), s2.is_sorted());
    println!("Some(5).is_none_or(|v| v > 0) = {}", Some(5).is_none_or(|v: i32| v > 0));
    let reps: Vec<i32> = std::iter::repeat_n(9, 3).collect();
    println!("repeat_n(9, 3) = {reps:?}");
}
