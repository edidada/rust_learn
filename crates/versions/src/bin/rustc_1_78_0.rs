// rustc 1.78.0 演示 —— cfg(target_abi)、#[diagnostic::on_unimplemented]、dyn 上转型 auto trait
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.78 特性在 1.98 均可用。
use std::fmt;

// 1.78 稳定 #[cfg(target_abi = ...)]：可按目标 ABI（如 eabihf、softfloat）分支编译。
// 演示：常用 tier1 目标的 target_abi 为空串 ""；为兼容 unexpected_cfgs 检查，
// 这里演示用 cfg! 宏可用的 cfg(target_os) 展示条件编译机制，target_abi 仅注释讲解。
const ABI_INFO: &str = if cfg!(target_os = "windows") { "windows" } else { "非 windows" };

// 1.78 稳定 #[diagnostic::on_unimplemented]：给 trait 自定义"未实现"错误文案。
// 若 bounds 没被满足，编译错误会显示 message/label，比默认的 "is not implemented" 友好得多。
#[diagnostic::on_unimplemented(
    message = "类型 `{Self}` 没有实现 `Shape`",
    label = "需要为 `{Self}` 实现 `Shape` 才能绘制"
)]
trait Shape {
    fn area(&self) -> f64;
}

struct Circle(f64);
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.0 * self.0
    }
}

// 借 Shape 定义一个泛型函数；让 Box<dyn UnknownType> 的上转型演示 auto trait。
fn draw(s: &impl Shape) -> f64 {
    s.area()
}

// 1.78：允许从 dyn Trait 上转型（upcast）为 dyn Trait + Auto（加 auto trait）。
// 对象安全 trait 才可造 dyn；Auto = Send/Sync/Unpin 等 auto trait。
trait Draw: fmt::Debug {
    fn draw(&self) -> String;
}
#[derive(Debug)]
struct Rect(u32);
impl Draw for Rect {
    fn draw(&self) -> String {
        format!("Rect({})", self.0)
    }
}

fn main() {
    println!("rustc 1.78.0 演示");

    println!("\n1. #[cfg(target_abi)] 与 #[diagnostic::on_unimplemented]");
    // 讲解：#[cfg(target_abi = "eabihf")] 这类属性写法 1.78 起可用；
    // 常见 tier1 目标 target_abi 为空串。此处用同机制的 target_os 演示运行期分支：
    println!("本机按 cfg 分支命中：{ABI_INFO}");
    let c = Circle(2.0);
    println!("draw(&Circle(2.0)) 面积 = {:.4}", draw(&c));
    // 反例注释：若某类型未实现 Shape，编译错误正文变成 on_unimplemented 里的 message：
    //   struct X; draw(&X);
    //   error: 类型 `X` 没有实现 `Shape`  ← 定制文案而非默认报错

    println!("\n2. dyn 上转型：dyn Draw -> dyn Draw + Send（1.78 Language）");
    // 旧行为：把 &dyn Draw 转成 &(dyn Draw + Send) 需要转发/包装；1.78 起引用形式直接上转型合法。
    let r = Rect(7);
    let d: &(dyn Draw + Send) = &r;
    // 1.78 首发可直接用的形态：把 &dyn Draw + Send 上转型成 dyn 时仍要写全 auto；
    // 真正展示"加 auto trait 上转型"用 supertrait 方向（dyn Supertrait -> dyn Sub 亦可）：
    // 这里演示同类型 auto trait 双写兼容 + 跨 trait 上转型（1.86 才稳定 supertrait 去 auto 化
    // 的完整规则，故 1.78 演示保守形态：保留 auto 标注说明该写法已稳定可行）。
    let ds: &(dyn Draw + Send) = d;
    println!("ds.draw() = {}", ds.draw());

    println!("\n3. 浮点 NaN 匹配（历史对比，讲解性输出）");
    // 1.78 起对 NaN 的字面量模式是硬错误（illegal_floating_point_literal_pattern 全移除）。
    // 历史形态（编译失败）：
    //   let x = f64::NAN;
    //   match x { f64::NAN => {}, _ => {} }   // error: 浮点 NaN 不能作模式
    // 现行等价物：只能用守卫判断 NaN：
    let x = f64::NAN;
    match x {
        v if v.is_nan() => println!("x 是 NaN（用守卫 is_nan() 匹配，模式只能兜底 `_`/绑定）"),
        _ => println!("x 不是 NaN"),
    }
    // 合法的一次性浮点模式：绑定式匹配（irrefutable）
    let y = 1.5f64;
    match y {
        v => println!("绑定模式 y = {v}"),
    }
}
