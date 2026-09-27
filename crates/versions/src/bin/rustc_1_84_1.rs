// rustc 1.84.1 演示 —— 补丁版：ICE、增量重叠 impl、下一代 trait solver 慢编译等修复
// 无新增稳定语言特性/API，仅以讲解性输出说明修复点。
trait Greeter {
    fn greet(&self) -> String;
}

struct A;
struct B;

// 1.84.1 修复的场景：incremental rebuild 下，下面这样"看似重叠"的 impl
// （一个针对具体类型、一个针对泛型但实际不相交）曾在增量编译时误报错误。
impl Greeter for A {
    fn greet(&self) -> String {
        "A".into()
    }
}

// 为演示 1.84.1 修复的"重叠 impl 增量误报"，写一个类型不重叠的泛型 impl：
trait Count {
    fn count(&self) -> usize;
}
impl<T> Count for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
}
impl Count for B {
    fn count(&self) -> usize {
        1
    }
}

fn main() {
    println!("rustc 1.84.1 演示（patch：修复，无新语言面）");

    println!("\n1. 重复 crate 诊断 ICE 132920 修复（讲解性）");
    println!("该修复针对 rustc 内部对同名/重复 crate 的诊断路径，不涉及语言语义。");

    println!("\n2. 增量重编译下的重叠 impl 报错修复");
    // 正常编译且共存的两个 impl（Vec<T> 泛型 vs B 具体类型，不相交）：
    let v = vec![1, 2, 3];
    println!("Vec count = {}", v.count());
    println!("B count = {}", B.count());
    // 1.84.1 之前：某些增量重编译场景会误报这两个 impl 重叠。

    println!("\n3. 下一代 trait solver 编译慢修复（讲解性）");
    println!("1.84.0 coherence 启用下一代 trait solver 后，部分工程编译显著变慢，1.84.1 修复。");

    println!("\n4. debuginfo / 源码构建修复（讲解性）");
    println!("LLVM location discriminator 超限时 debuginfo 不再损坏；构建系统系列修复。");
}
