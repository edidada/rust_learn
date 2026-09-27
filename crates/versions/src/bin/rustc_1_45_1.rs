// rustc 1.45.1 演示 —— patch 版 bugfix：const 传播与引用 / rustfmt cfg_attr / 隐式 region bound / x.py install clippy
// 说明：本版无新增语言/库特性，以下节对应笔记中的 Patch Fixes，用现行稳定写法演示相关机制。

// 1.45.1 修复背景：上一版引入的“引用参与常量传播（const propagation of references）”
// 在某些代码上会错误地允许生成非法代码；本版收回这一错误判断，恢复保守行为。
// 下面流程函数只做带引用的 const/变量求值演示，实际修复在编译器 const 检查层。
const BASE: usize = 4;

fn main() {
    println!("rustc 1.45.1 演示（patch：仅修复，无新增稳定 API）");

    println!("\n1. 修复带引用的 const 传播（const propagation with references）");
    // 历史：某次变更后编译器试图把引用也纳入 const propagation，可能算错/通过不该通过的代码；
    // 1.45.1 修复判断，保证含引用的情况按保守规则处理。
    let local = BASE + 1; // 每次调用本地具体值；引用 LifeCycle 上编译器保守处理
    let r: &usize = &local; // 引用变量的 const 传播被 1.45.1 修复为正确行为
    println!("const BASE = {}, 引用 r = {}", BASE, r);
    println!("机制：const 传播优化不再把引用错误地视为可完全求值的操作数。");

    println!("\n2. rustfmt 再次接受 cfg_attr 中的 rustfmt_skip");
    // 背景：rustfmt 曾拒绝 `#[cfg_attr(..., rustfmt_skip)]` 的嵌套写法，1.45.1 恢复；
    // 该问题在 1.44.1 也修过，本版为复现问题的再次修复（同 patch 系列的一部分）。
    println!("机制：`#[cfg_attr(feature = \"x\", rustfmt_skip)]` 这样的属性再次被 rustfmt 接受。");
    println!("（rustfmt 属工具链行为，不在运行期演示）");

    println!("\n3. 避免虚假的隐式 region bound");
    // 背景：1.45.0 的某个改动可能引入了不应存在的隐式生命周期/region 约束，
    // 导致个别 trait 方法此前能编译、现在/本版之后行为更严格且正确。
    // 用现行稳定写法验证生命周期约束仍按预期工作：
    let s = String::from("region bound");
    let sliced: &str = s.as_str(); // 'a 生命周期正常推断，不引入虚假 bound
    println!("字符串切片 = {:?}，生命周期推断依旧正确", sliced);

    println!("\n4. x.py install 时安装 clippy");
    // 背景：官方构建脚本 x.py 的 `install` 此前不会一并安装 clippy，1.45.1 补齐；
    // 属构建发行流程修复，对正常运行 rustc 的一般项目无观感影响。
    println!("机制：源码构建者执行 `x.py install` 后 clippy 一并可用，便于用 rustup component add clippy 对照。");
}
