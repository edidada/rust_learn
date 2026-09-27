// rustc 1.29.0 演示 —— Iterator::flatten + Rc/Arc::downcast
// 该版本 introduces:
// 1) Iterator::flatten 稳定：嵌套迭代器一键拍平（此前需 chain/match 手写）。
// 2) Rc::downcast / Arc::downcast 稳定：把 Rc<dyn Any> / Arc<dyn Any> 转回具体类型。
// 3) 弃用 str::slice_unchecked（改用 get_unchecked）、std::env::home_dir。
// 4) cargo fix：自动把 2015 版代码迁移到 2018 版（工具讲解）。

use std::any::Any;
use std::rc::Rc;

fn main() {
    println!("rustc 1.29.0 演示");

    // 1. Iterator::flatten
    println!("\n1. Iterator::flatten");
    let nested = vec![vec![1, 2], vec![3], vec![]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("vec![vec![1,2],vec![3],vec![]].flatten() = {:?}", flat);

    // Option 的迭代也可 flatten（Some 展开为元素，None 为空）
    let opts = vec![Some(10), None, Some(30)];
    let flat2: Vec<i32> = opts.into_iter().flatten().collect();
    println!("Option 迭代 flatten = {:?}", flat2);

    // 2. Rc::downcast / Arc::downcast
    println!("\n2. Rc::downcast / Arc::downcast");
    let r: Rc<dyn Any> = Rc::new(String::from("装箱的字符串"));
    if let Ok(rs) = Rc::downcast::<String>(r) {
        println!("downcast 回 String: {}", rs);
    }
    let a: std::sync::Arc<dyn Any + Send + Sync> =
        std::sync::Arc::new(42i32);
    if let Ok(aint) = std::sync::Arc::downcast::<i32>(a) {
        println!("Arc::downcast 回 i32: {}", aint);
    }

    // 3. 弃用 API 说明（不调用，仅讲解）
    println!("\n3. 1.29 弃用清单（讲解）");
    // 旧：unsafe { "abc".slice_unchecked(0, 2) }  → 新："abc".get_unchecked(0..2)
    println!("str::slice_unchecked 弃用 → str::get_unchecked(begin..end)");
    println!("std::env::home_dir 弃用 → dirs crate 的 home_dir");
    println!("cargo fix 已加入：自动迁移 2015 -> 2018 edition");
}
