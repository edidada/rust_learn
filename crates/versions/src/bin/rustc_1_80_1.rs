// rustc 1.80.1 演示 —— 补丁版：jump threading 浮点误编译修复 + dead_code lint 回滚
// 无新增稳定语言特性/API，因此仅以讲解性输出说明两个修复点。
fn main() {
    println!("rustc 1.80.1 演示（patch：修复与回滚，无新语言面）");

    println!("\n1. 修复 jump threading MIR 优化中浮点比较的误编译");
    // 历史形态（1.80.0 受影响）：下面这类"比较结果决定分支"的代码，在特定
    // 优化路径下浮点比较可能被错误折叠。1.80.1 修复。
    let a = 1.0f64;
    let b = f64::NAN; // NaN 与任何比较都为 false
    let r = if a < b { "lt" } else if a > b { "gt" } else { "unordered/eq" };
    println!("1.0 与 NaN 比较 → {r}（期望 unordered/eq）");

    println!("\n2. 回滚 1.80.0 对 dead_code lint 的变更");
    // 1.80.0 曾让 dead_code 对间接使用的项（如仅通过 trait/泛型触达）误报，
    // 1.80.1 恢复旧行为：以下仅被泛型代码间接调用的函数不再被报 dead_code。
    struct Tag;
    impl Tag {
        fn helper(&self) -> u32 {
            42 // 仅通过下面的泛型函数间接使用
        }
    }
    fn indirect<T>(t: &T) -> u32
    where
        T: Fn() -> u32,
    {
        t()
    }
    // 为演示而包装成 Fn：
    let out = indirect(&|| Tag.helper());
    println!("间接调用的 helper() = {out}（dead_code 不再误报）");
}
