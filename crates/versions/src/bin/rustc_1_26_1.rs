// rustc 1.26.1 演示 —— patch 版：收回三个 1.26.0 里的错误稳定
// 该版本 introduces:
// 1) `fn main() -> impl Trait` 被收回（非 Termination trait 不能作为 main 返回）。
// 2) const-fn 中 `NaN > NaN` 不再返回 true（此前错误地为 true）。
// 3) 方法参数里的 `impl Trait` 禁用 turbofish `::<T>` 形式。
// 另：RLS 修复 Windows 可用性、Rustfmt 修复排版。语言本体补丁演示用 println。

fn main() {
    println!("rustc 1.26.1 演示");
    println!("\n1. 收回的三个 1.26.0 误稳定（讲解）");
    println!("a. fn main() -> impl Trait 禁用");
    println!("b. const 上下文里 NaN > NaN == false（此前 buggy true）");
    println!("c. impl Trait 方法参数禁 turbofish：foo::<T>() 不再合法");
    println!("\n2. 1.26.0 特性仍正常（..= 检查）");
    let s: i32 = (0..=3).sum();
    println!("(0..=3).sum() = {}", s);
    // const 上下文 NaN 比较验证：
    const _OK: bool = { let nan = f64::NAN; nan == nan }; // NaN != NaN，常量折叠为 false
    println!("const 上下文 NaN==NaN = {}", { let nan = f64::NAN; nan == nan });
}
