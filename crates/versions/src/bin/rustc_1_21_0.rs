// rustc 1.21.0 演示 —— 'static 字面量引用 + mem::discriminant
// 该版本 introduces:
// 1) Language：字面量可以直接提升为 'static 引用（此前需要 const/static 绑定中转）。
// 2) 唯一稳定化的库 API：std::mem::discriminant —— 拿到枚举判别式，比较"是否同一变体"。
// 3) 路径语法放宽：Vec::<i32>::new() 中 :: 的可选性在所有上下文可用。

use std::mem::discriminant;

fn main() {
    println!("rustc 1.21.0 演示");

    // 1. 'static 字面量引用：&0 被提升到静态存储
    println!("\n1. 字面量的 'static 引用");
    let x: &'static u32 = &0;
    println!("x = {}（&'static u32 可直接指向字面量）", x);

    // 2. mem::discriminant：比较两枚举值是否同一变体（不看字段值）
    println!("\n2. mem::discriminant");
    enum State {
        Idle,
        Run(u32),
        Stop,
    }
    let a = State::Run(1);
    let b = State::Run(99);
    let c = State::Idle;
    // discriminant 只区分变体，Run(1) 和 Run(99) 的判别式相等
    println!("Run(1) 与 Run(99) 同变体? {}", discriminant(&a) == discriminant(&b));
    println!("Run(1) 与 Idle 同变体? {}", discriminant(&a) == discriminant(&c));

    // 3. 放宽路径语法：Vec::<i32>::new()（turbofish 前的 :: 现在任何上下文都可省写）
    println!("\n3. 放宽的路径语法");
    let v = Vec::<i32>::new(); // 1.21 前：在部分上下文必须写成 ::Vec::<i32> 或受限
    println!("Vec::<i32>::new() -> len = {}", v.len());

    // 4. Rc/Arc 的 From 转换：From<str> / From<Vec<T>> / From<&[T]>
    println!("\n4. Rc/Arc 新 From 实现");
    let r: std::rc::Rc<str> = "字面量转 Rc<str>".into();
    let a2: std::sync::Arc<str> = String::from("String 转 Arc<str>").into();
    let data = [1i32, 2, 3];
    let r2: std::rc::Rc<[i32]> = data[..].into(); // From<&[T]>（T: Clone）
    println!("{} | {} | {:?}", r, a2, r2);
}
