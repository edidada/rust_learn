//! 知识点：借用检查器行为的历史遗留问题
//!
//! 无处不在的生命周期和所有权耦合，牵一发而动全身。这也是为什么有人热衷于
//! "到处重写"而不是开创新领域——新领域的代码会面临不停的重构，而 Rust
//! 天生对代码改动不友好：同一段代码可能因 Edition 或编译器版本不同而
//! 得到不同的诊断结果。
//!
//! 即使在同一 Edition 内，借用检查器本身也在演进：
//! · 对"移动后部分重新赋值"的行为，2015 Edition 早期只是一个警告
//! · 后来（现代 rustc）变成硬性错误
//! · 2018 Edition 则一直是错误
//!
//! 以下代码演示相关模式在现代编译器下的行为；被注释掉的行是会触发
//! E0382（use of moved value / borrow of partially moved value）的写法。

#[derive(Debug)]
struct P {
    name: String,
    age: i32,
}

fn main() {
    println!("=== 借用检查器行为的历史遗留 ===\n");

    // 1. 移动后整体重新赋值再使用
    println!("1. 移动后重新赋值 (Move then Reassign)");
    reassign_after_move();

    // 2. 部分移动后重新初始化该字段
    println!("\n2. 部分移动后部分重新赋值 (Partial Move then Reinit)");
    partial_reinit();

    // 3. 重新赋值救不回"没有重新赋值的使用"
    println!("\n3. 移动后未重新赋值就使用 (硬性错误)");
    use_after_move_is_error();

    // 4. Edition 之间的差异是"重构不友好"的来源之一
    println!("\n4. Edition 差异与重构");
    edition_differences();
}

// 1. 移动后整体重新赋值：重新赋值会覆盖"已移动"状态，之后可以正常使用
fn reassign_after_move() {
    let mut s = String::from("a");
    let t = s; // s 被移动
    s = String::from("b"); // 重新赋值：s 重新有效
    println!("   s = {}, t = {}", s, t);
}

// 2. 部分移动后，重新初始化被移走的字段，整体又变回完整可用状态
fn partial_reinit() {
    let mut p = P {
        name: String::from("a"),
        age: 1,
    };
    let n = p.name; // 部分移动：p.name 被移走
    println!("   移动后未移动字段仍可用: p.age = {}", p.age);
    // println!("{:?}", p); // ❌ 此时整体使用会报错（部分移动未修复）

    p.name = String::from("b"); // 部分重新赋值：把被移走的字段补回来
    println!("   重新赋值后整体又完整: {:?}", p);
    println!("   被移走的 name = {}", n);
}

// 3. 只有"重新赋值"能让变量重新有效；不重新赋值就使用是硬性错误
fn use_after_move_is_error() {
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("{:?}", v); // ❌ E0382: use of moved value: `v`
    //
    // 在 2015 Edition 早期，这类"移动后再使用"的诊断曾只是警告，
    // 后来变成硬性错误；2018 Edition 则一直是错误。
    println!("   只能使用新所有者 v2 = {:?}", v2);

    let p = P {
        name: String::from("a"),
        age: 1,
    };
    let n = p.name;
    let _ = n;
    // println!("{:?}", p); // ❌ E0382: borrow of partially moved value: `p`
    println!("   部分移动后整体借用是硬性错误，只能访问未移动字段 p.age = {}", p.age);
}

// 4. 同一段涉及部分移动和闭包/生命周期的代码，可能在不同 Edition 下
//    编译结果不同（详见 2021 分支的闭包捕获、2024 分支的临时值作用域）。
fn edition_differences() {
    println!("   闭包捕获规则变化（2021）与临时值作用域收紧（2024）");
    println!("   会让同一段代码在旧 Edition 编译失败、新 Edition 通过（或反之）");
    println!("   生命周期/所有权与 Edition 演进耦合 → 代码改动不友好");
}
