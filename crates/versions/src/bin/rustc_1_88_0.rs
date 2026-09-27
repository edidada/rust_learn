// rustc 1.88.0 演示 —— let chains（重点，edition 2024）、select_unpredictable、Cell::update、as_chunks
// 注意：本仓库为 edition 2024，let chains 可直接编译运行。
use std::cell::Cell;
use std::collections::HashMap;
use std::hint::select_unpredictable;

fn main() {
    println!("rustc 1.88.0 演示");

    println!("\n1. let chains：`if let … && let …`（1.88 重点）");
    // 旧写法（嵌套 if let）：
    //   if let Some(a) = opt1 { if let Some(b) = opt2 { ... } }
    // let chains（edition 2024 起）：
    let o1: Option<&str> = Some("config");
    let o2: Option<u8> = Some(3);
    if let Some(name) = o1 && let Some(n) = o2 && n < 5 {
        // let 子表达式可以与布尔条件 n < 5 混用！
        println!("let chain 全部命中：name = {name}, n = {n}");
    }
    // 可反驳模式 + 混合布尔：
    let mixed: Result<i32, &str> = Ok(10);
    if let Ok(v) = mixed && v > 5 && o2.is_some() {
        println!("Ok({v}) && v>5 && is_some() —— let 与布尔任意交错");
    }
    // 反例演示（链中某 let 失败 → 整体走 else）：
    let bad: Option<u8> = None;
    if let Some(_n) = o2 && let Some(_) = bad {
        println!("不会走到这里");
    } else {
        println!("链上 let Some(_) = bad 失败 → else 分支");
    }

    println!("\n2. let chains：`while let … && …`");
    let mut it = [Some(1), Some(2), None, Some(3)].into_iter();
    let mut acc = 0;
    while let Some(v) = it.next() && let Some(x) = v {
        acc += x; // None 一出现循环即止
    }
    println!("while let 累计 = {acc}（遇 None 提前结束）");

    println!("\n3. hint::select_unpredictable（1.88 稳定）");
    // 语义：提示编译器该分支选择"不可预测"，与 std::hint::branch_hint 配合，
    // 用于把可预测性交给 CPU 的场景；签名 select_unpredictable(b, x, y) -> T。
    let v = select_unpredictable(true, 111, 222);
    println!("select_unpredictable(true, 111, 222) = {v}");

    println!("\n4. Cell::update（1.88 稳定）");
    let c = Cell::new(10);
    // update：读旧值 → f → 写回（Cell 的"读改写"原子化到方法层面）：
    let _old = c.update(|v| v + 5);
    println!("Cell::update(+5)：新值 = {}", c.get());

    println!("\n5. HashMap::extract_if（1.88 稳定）");
    let mut m = HashMap::from([("a", 1), ("bb", 2), ("c", 3)]);
    let removed: Vec<(&str, i32)> = m.extract_if(|k, _| k.len() > 1).collect();
    println!("extract_if(len>1) 移走 = {removed:?}, 剩余 = {m:?}");

    println!("\n6. <[_]>::as_chunks / as_rchunks（1.88 稳定）");
    let data = [1, 2, 3, 4, 5, 6, 7];
    let (chunks, rest) = data.as_chunks::<3>(); // 按前向分块：[[1,2,3],[4,5,6]] + [7]
    println!("as_chunks::<3>() = {chunks:?}, 余 = {rest:?}");
    let (head, rchunks) = data.as_rchunks::<3>(); // 按后向分块：[1] + [[2,3,4],[5,6,7]]
    println!("as_rchunks::<3>() = 头 = {head:?}, 块 = {rchunks:?}");

    println!("\n7. cfg 布尔字面量（1.88 稳定 feature：cfg_boolean_literals）");
    // #[cfg(true)] / #[cfg(false)] 现在是合法谓词（此前只能写 #[cfg(all())] 等 hack）。
    #[cfg(false)]
    println!("这行不会编译进来");
    #[cfg(true)]
    println!("#[cfg(true)] 分支已编译进来");
    // 裸指针 Default（1.88 稳定）：Default for *const T / *mut T 为 null：
    let p: *const i32 = Default::default();
    println!("*const i32::default() = {:?}（空指针）", p.is_null());
}
