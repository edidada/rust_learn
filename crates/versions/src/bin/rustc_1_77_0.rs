// rustc 1.77.0 演示 —— C 字符串字面量、offset_of!、first_chunk 系列、clear_poison 等
fn main() {
    println!("rustc 1.77.0 演示");

    // ============================================================
    println!("\n1. C 字符串字面量 c\"...\"");
    // 1.77 稳定：c"..." 直接是 &'static CStr，比 CStr::from_bytes_with_nul 方便
    let cs = c"hello 1.77";
    println!("  c\"hello 1.77\" -> {:?} -> {}", cs, cs.to_string_lossy());
    let empty_cs = c"";
    println!("  c\"\" 为空: {}", empty_cs.is_empty());

    // ============================================================
    println!("\n2. mem::offset_of!");
    #[allow(dead_code)]
    struct S {
        a: u8,
        b: u32,
        c: u64,
    }
    let oa = std::mem::offset_of!(S, a);
    let ob = std::mem::offset_of!(S, b);
    let oc = std::mem::offset_of!(S, c);
    println!("  S 的字段偏移: a={}, b={}, c={}（含对齐填充）", oa, ob, oc);

    // ============================================================
    println!("\n3. slice first_chunk / split_first_chunk 系列");
    let v = [1, 2, 3, 4, 5];
    let (head2, rest) = v.split_first_chunk::<2>().unwrap();
    println!("  split_first_chunk::<2> = {:?} + {:?}", head2, rest);
    let (tail1, rest2) = v.split_last_chunk::<1>().unwrap();
    println!("  split_last_chunk::<1> = {:?} + {:?}", tail1, rest2);
    let first3: Option<&[i32; 3]> = v.first_chunk();
    println!("  first_chunk::<3> = {:?}", first3);
    let too_many: Option<&[i32; 9]> = v.first_chunk();
    println!("  first_chunk::<9> 长度不足 = {:?}", too_many);

    // ============================================================
    println!("\n4. slice::chunk_by");
    let runs: Vec<i32> = vec![1, 1, 2, 2, 3];
    let chunks: Vec<Vec<i32>> = runs.chunk_by(|a, b| a == b).map(|c| c.to_vec()).collect();
    println!("  相邻相等分块 = {:?}", chunks); // [[1,1], [2,2], [3]]

    // ============================================================
    println!("\n5. array::each_ref");
    let arr = [10, 20, 30];
    let refs = arr.each_ref(); // [&i32; 3]：一次性借出全数组引用视图
    println!("  each_ref = {:?}", refs);
    // 配合解构定长引用：let [a, b, _] = arr.each_ref();

    // ============================================================
    println!("\n6. f64::round_ties_even");
    println!("  0.5.round_ties_even() = {}", 0.5f64.round_ties_even()); // 0
    println!("  1.5.round_ties_even() = {}", 1.5f64.round_ties_even()); // 2
    println!("  2.5.round_ties_even() = {}", 2.5f64.round_ties_even()); // 2

    // ============================================================
    println!("\n7. Bound::map");
    use std::ops::Bound::{self, Excluded, Included};
    let b: Bound<i32> = Included(4);
    let b2: Bound<String> = b.map(|v| v.to_string());
    println!("  Bound(4) -> map(|x| x.to_string()) = {:?}", b2);
    let e: Bound<i32> = Excluded(1);
    println!("  Excluded(1).map 转字符串 = {:?}", e.map(|v| v * 2 + 1).map(|v| format!("<{}", v)));

    // ============================================================
    println!("\n8. File::create_new");
    use std::fs::File;
    let tmp = std::env::temp_dir().join("rust_1770_create_new.txt");
    let r = File::create_new(&tmp);
    println!("  首次 create_new = {:?}", r.is_ok());
    let r2 = File::create_new(&tmp);
    println!("  再次 create_new = Err（文件已存在）: {}", r2.is_err());
    drop(r);
    let _ = std::fs::remove_file(&tmp);

    // ============================================================
    println!("\n9. Mutex::clear_poison / RwLock::clear_poison");
    let m = std::sync::Mutex::new(0i32);
    {
        // 正常锁定说明：即使曾发生毒化（panic 时锁定），clear_poison 也能恢复
        let _guard = m.lock().unwrap();
    }
    m.clear_poison();
    println!("  Mutex::clear_poison() 调用成功（毒化后可主动恢复）");
    let rw = std::sync::RwLock::new(0i32);
    let _cleared_rw = rw.clear_poison();
    println!("  RwLock::clear_poison() 调用成功");

    // ============================================================
    println!("\n10. 文字说明");
    println!("  - static_mut_refs lint：对可变 static 的引用开始警告（&MUT 会被提示）");
    println!("  - let-else 中大括号宏调用被 deny");
    println!("  - build script 指令新语法 cargo::；core::net 让 no_std 也能用网络类型");
}
