//! 知识点：Rust 默认移动（move）规则
//!
//! 核心一句话：赋值、传参、返回时，默认都是移动语义，而非拷贝。
//!
//! 移动的本质：不是内存拷贝，只是所有权转移。编译器在语义层面把原变量
//! 标记为"已移动"，底层数据（如堆上的字节）原地不动，只是换了个所有者。
//! 这也是 Rust 零成本抽象的一部分。
//!
//! 一句话总结：默认移动 = 所有权转移，原变量失效；只有实现了 Copy 的类型
//! 才自动复制。需要保留原值时，用 clone() 或借用 &。

fn main() {
    println!("=== Rust 默认移动规则 ===\n");

    // 1. 赋值即移动
    println!("1. 赋值即移动 (Assignment)");
    assignment_moves();

    // 2. 传参即移动
    println!("\n2. 传参即移动 (Function Parameter)");
    parameter_moves();

    // 3. 返回即移动
    println!("\n3. 返回即移动 (Return)");
    return_moves();

    // 4. Copy 与 Move 的分界
    println!("\n4. Copy 与 Move 的分界");
    copy_vs_move();

    // 5. 如何避免移动
    println!("\n5. 如何避免移动");
    avoid_move();

    // 6. 常见易错点
    println!("\n6. 常见易错点");
    common_pitfalls();
}

// 1. 赋值即移动
fn assignment_moves() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动进 s2
    // println!("{}", s1); // ❌ 编译错误：use of moved value: `s1`
    println!("   s2 = {}", s2);

    let x = 5;
    let y = x; // i32 实现了 Copy：按位复制，x 仍可用
    println!("   x = {}, y = {} (Copy 类型)", x, y);
}

// 2. 传参即移动
fn take(s: String) {
    // s 移动进函数
    println!("   take() 拥有了: {}", s);
}

fn parameter_moves() {
    let s = String::from("hi");
    take(s);
    // println!("{}", s); // ❌ s 已失效（所有权已交给 take）
    println!("   s 已移动进 take()，原变量不能再用");
}

// 3. 返回即移动
fn give() -> String {
    let s = String::from("hi");
    s // s 移动出函数（返回）
}

fn return_moves() {
    let g = give();
    println!("   give() 把所有权交还出来: {}", g);
}

// 4. Copy 与 Move 的分界：关键在于类型是否实现了 Copy
fn copy_vs_move() {
    // 实现 Copy 的类型（栈上固定大小）：赋值/传参时按位复制，原变量仍可用
    // · 整数、浮点、bool、char
    // · 共享引用 &T
    // · 由以上类型组成的元组/数组（如 (i32, bool)）
    let flag = true;
    let flag2 = flag;
    println!("   bool 是 Copy: flag = {}, flag2 = {}", flag, flag2);

    let pair = (1, true);
    let pair2 = pair; // (i32, bool) 全部字段 Copy → 整体 Copy
    println!("   (i32, bool) 是 Copy: {:?}", pair2);

    let shared = &flag;
    let shared2 = shared; // &T 是 Copy（复制的只是引用本身）
    println!("   &T 是 Copy: {} {}", shared, shared2);

    // 未实现 Copy 的类型（拥有堆资源或需析构）：移动，原变量失效
    // · String、Vec<T>、Box<T>
    // · 可变引用 &mut T（本身不可 Copy）
    let v = vec![1, 2, 3];
    let v2 = v; // Move：Vec 拥有堆内存
    // println!("{:?}", v); // ❌
    println!("   Vec<T> 是 Move: {:?}", v2);

    let mut num = 1;
    let r = &mut num; // &mut T 不可 Copy（同一时刻只能有一个可变借用）
    *r += 1;
    // let r2 = &mut num; // ❌ 同时只能有两个可变借用
    println!("   &mut T 不可 Copy: num = {}", num);
}

// 5. 如何避免移动
fn borrow_len(s: &String) -> usize {
    s.len() // &T 只读借用，不转移所有权
}

fn borrow_append(s: &mut String) {
    s.push_str(", world"); // &mut T 可变借用，不转移所有权
}

fn avoid_move() {
    // 方式一：Clone —— 显式深拷贝
    let s1 = String::from("hi");
    let s2 = s1.clone();
    println!("   clone: s1 = {}, s2 = {}", s1, s2);

    // 方式二：借用 &T —— 只读借用
    let s = String::from("hello");
    let len = borrow_len(&s);
    println!("   借用 &T: '{}' 长度 {}，s 仍归我所有", s, len);

    // 方式三：借用 &mut T —— 可变借用
    let mut s = String::from("hello");
    borrow_append(&mut s);
    println!("   借用 &mut T: {}", s);

    // 方式四：Copy —— 仅适用于可按位复制的简单类型
    let n = 42;
    let m = n; // 自动 Copy，无需额外操作
    println!("   Copy: n = {}, m = {}", n, m);
}

// 6. 常见易错点
fn common_pitfalls() {
    // 易错点 1：移动后还想用
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("{:?}", v); // ❌ use of moved value: `v`
    println!("   移动后还想用 → 只能用 v2 = {:?}", v2);

    // 易错点 2：循环中移动（for 循环会消费集合本身）
    let words = vec![String::from("a"), String::from("b")];
    for s in &words {
        // 借用，而不是移动
        println!("   for s in &words: {}", s);
    }
    println!("   words 仍有效: {:?}", words);
    // for s in words { } // ❌ words 被移动进循环，之后不能再用

    // 易错点 3：部分移动
    let p = Person {
        name: String::from("a"),
        age: 1,
    };
    let n = p.name; // 部分移动：p.name 被移走
    // println!("{:?}", p); // ❌ borrow of partially moved value: `p`
    println!("   p.age = {} 未移动的字段仍可用", p.age);
    println!("   被移走的 name = {}", n);

    // 易错点 4：移动与 Drop 冲突
    // 实现了 Drop 的类型不能部分移动，编译器会直接拒绝：
    let d = Dropper {
        s: String::from("x"),
    };
    // let moved = d.s; // ❌ E0509: cannot move out of type `Dropper`,
    //                   //          which implements the `Drop` trait
    println!("   实现 Drop 的类型不能部分移动: d.s = {}", d.s);
}

struct Person {
    name: String,
    age: i32,
}

struct Dropper {
    s: String,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("   Dropper 被析构");
    }
}
