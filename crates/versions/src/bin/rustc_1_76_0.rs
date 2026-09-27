// rustc 1.76.0 演示 —— unwrap_or_clone、inspect、ptr::addr_eq、type_name_of_val
fn main() {
    println!("rustc 1.76.0 演示");

    // ============================================================
    println!("\n1. Arc::unwrap_or_clone / Rc::unwrap_or_clone");
    use std::rc::Rc;
    use std::sync::Arc;
    // 独占：直接取出内部值，无需 match Rc::try_unwrap(...).unwrap_or_else(...)
    let only: Arc<String> = Arc::new("独占".to_owned());
    println!("  Arc 独占 -> unwrap_or_clone = {}", Arc::unwrap_or_clone(only));
    let shared = Arc::new("共享".to_owned());
    let _clone = shared.clone();
    println!("  Arc 共享(2) -> unwrap_or_clone = {}", Arc::unwrap_or_clone(shared).len());
    let r1: Rc<String> = Rc::new("Rc".to_owned());
    println!("  Rc -> unwrap_or_clone = {}", Rc::unwrap_or_clone(r1));

    // ============================================================
    println!("\n2. Option::inspect / Result::inspect(_err)");
    // inspect：透明查看值但不改变流程，比 map(|x| { log; x }) 更直白
    let mut seen = 0;
    let x = Some(7).inspect(|v| seen = *v);
    println!("  Option::inspect 看到 {}，结果仍为 {:?}", seen, x);
    let r = Ok::<i32, &str>(3).inspect(|v| println!("  Result::inspect 成功值 = {}", v));
    let e = Err::<i32, &str>("失败").inspect_err(|err| println!("  Result::inspect_err = {}", err));
    println!("  链式结果: {:?} / {:?}", r, e);

    // ============================================================
    println!("\n3. ptr::from_ref / from_mut / ptr::addr_eq");
    use std::ptr;
    let v = 10;
    let p = ptr::from_ref(&v); // 引用 -> 裸指针（现代写法）
    let m: *mut i32 = ptr::from_mut(&mut { v });
    println!("  from_ref 读值 = {}", unsafe { *p });
    println!("  addr_eq(&v, &v) = {}（仅比地址）", ptr::addr_eq(&v, &v));
    let other = 10;
    println!("  addr_eq(两个不同变量的引用) = {}", ptr::addr_eq(&v, &other));
    println!("  from_mut 可用性已确认（unsafe 读取略）");
    let _ = m;

    // ============================================================
    println!("\n4. type_name_of_val");
    let s = "hello";
    let v2 = vec![1, 2, 3];
    println!("  &str 的类型名 = {}", std::any::type_name_of_val(&s));
    println!("  Vec<i32> 的类型名 = {}", std::any::type_name_of_val(&v2));

    // ============================================================
    println!("\n5. std::hash 顶层导出（DefaultHasher / RandomState）");
    // 以前要从 std::collections::hash_map 引入；1.76 起可直接用
    use std::hash::{BuildHasher, DefaultHasher, Hasher, RandomState};
    // DefaultHasher::new() + Hasher::write/finish（顶层导出后可直接使用）
    let mut hasher = DefaultHasher::new();
    hasher.write(b"key");
    let digest = hasher.finish();
    println!("  DefaultHasher 顶层导出可用，hash = {}", digest);
    // RandomState 也可顶层导入，配合 hash_one 一步求值
    let one: u64 = RandomState::new().hash_one("rust 1.76");
    println!("  RandomState::new().hash_one(\"rust 1.76\") = {}", one);

    // ============================================================
    println!("\n6. 文字说明");
    println!("  - dbg!() 输出加入列号，方便定位");
    println!("  - Rust ABI 兼容性正式文档化：char == u32 等");
    println!("  - ambiguous_wide_pointer_comparisons：比较宽指针（如 dyn Trait）会提示");
    println!("    vtable 地址比较可能产生歧义，建议用 ptr::addr_eq 只比数据地址");
}
