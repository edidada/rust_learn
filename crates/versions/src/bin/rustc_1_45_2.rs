// rustc 1.45.2 演示 —— patch 版 bugfix：tuple struct 模式绑定 / #[track_caller] 与 trait object
// 说明：本版无新增语言/库特性，以下节对应笔记中的 Patch Fixes，用现行稳定写法演示相关机制。

// 1.45.2 修复背景：tuple struct 模式里的变量绑定此前可能编译出错/绑错位置，本版修复。
struct Point(i32, i32);

// 1.45.2 修复背景：#[track_caller] 与 trait object 的集成问题——通过 trait object
// 调用带 #[track_caller] 的方法时，caller 位置信息可能丢失；本版修复。
// 现行稳定形态演示：
trait Caller {
    // trait 方法也可以标 track_caller；1.45.2 修复了与 trait object 动态派发的集成
    #[track_caller]
    fn where_am_i(&self) -> &'static str {
        // 历史形态（1.45.2 修复中）：这里原先不能用 Location::caller() 与 trait object 正常协作；
        // 现行写法直接调用 panic 宏即可自动带上调用者行号。
        panic!("called from caller info demo");
    }
}

struct Impl;
impl Caller for Impl {}

fn main() {
    println!("rustc 1.45.2 演示（patch：仅修复，无新增稳定 API）");

    println!("\n1. 修复 tuple struct 模式中的绑定");
    // 1.45.2 之前，形如下方的解构可能在个别模式里报错或绑错字段；修复后正常。
    let p = Point(1, 2);
    let Point(x, y) = p; // tuple struct 模式的绑定
    println!("Point({}, {}) 解构 = x:{}, y:{}", p.0, p.1, x, y);
    // 嵌套 tuple struct 模式同样正常：
    let nested = (Point(3, 4), Point(5, 6));
    let (Point(a, b), Point(c, d)) = nested;
    println!("嵌套解构 = ({},{}), ({},{})", a, b, c, d);

    println!("\n2. 修复 #[track_caller] 与 trait object 的集成");
    // 1.45.2 修复后：通过 &dyn Caller 调用 where_am_i，panic 消息正确指向调用处行号；
    // 修复仅仅影响位置信息正确性，不影响 API 形态。
    let obj: &dyn Caller = &Impl;
    // 为了不 panic 给读者看，我们说明机制：下面的 panic 若真的触发，位置应是调用那一行。
    println!("机制：#[track_caller] 让函数内部 panic 时携带调用者位置；");
    println!("trait object 上的方法带该属性时，位置同样能正确传播（1.45.2 修复点）。");
    let _ = obj; // 演示类型，不真正调用
    println!("演示方式：trait Caller 上标注 #[track_caller] 的 fn where_am_i，");
    println!("若在 main 的某一行调用，panic 位置就是 main 那一行——而非 trait 方法内部。");
}
