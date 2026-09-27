// rustc 1.74.1 演示 —— patch 版本：LLVM 访问违例、mem::discriminant 保证等（文字说明）
fn main() {
    println!("rustc 1.74.1 演示");
    println!("\n1. Changes（编译器侧修复）");
    println!("  - 解决 LLVM 中偶发的 STATUS_ACCESS_VIOLATION");
    println!("  - 澄清 std::mem::discriminant 的保证：Discriminant<T> 不依赖 T 的生命周期");
    println!("  - 修复子类型相关回归");
    // 演示 discriminant 正常用法（不受本补丁影响）
    enum E { A, B }
    println!("  discriminant(E::A) == discriminant(E::A) -> {}",
        std::mem::discriminant(&E::A) == std::mem::discriminant(&E::A));
}
