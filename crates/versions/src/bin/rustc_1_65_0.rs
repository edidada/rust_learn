// rustc 1.65.0 演示 —— let-else、GATs、label-break-value 等稳定化
fn main() {
    println!("rustc 1.65.0 演示");

    // ============================================================
    println!("\n1. let-else 稳定化");
    // 1.65 稳定了 `let PAT = EXPR else { ... }`：
    // 模式不匹配时执行 else 块，else 块必须"发散"（return/break/panic 等）。
    // 对比旧行为：以前要写 `let x = match ... { None => return, Some(v) => v };` 或嵌套 if let。
    let values: Vec<Option<i32>> = vec![Some(3), None, Some(7)];
    let mut ok = Vec::new();
    for v in values {
        // Some(v) 匹配则取 v；None 则走 else 分支 continue
        let Some(x) = v else {
            println!("  遇到 None，走 else 分支 -> continue");
            continue;
        };
        ok.push(x);
    }
    println!("  成功解包: {:?}", ok); // [3, 7]

    // ============================================================
    println!("\n2. GATs 泛型关联类型");
    // 1.65 稳定 GATs：关联类型可以带自己的泛型参数/生命周期。
    // 经典用例 LendingIterator：Item<'a> 借用 &mut self，迭代器借出借用，逐个返回。
    trait LendingIterator {
        type Item<'a> where Self: 'a;
        fn next(&mut self) -> Option<Self::Item<'_>>;
    }
    struct Counter(usize);
    impl LendingIterator for Counter {
        type Item<'a> = &'a mut usize where Self: 'a;
        fn next(&mut self) -> Option<Self::Item<'_>> {
            // 返回对内部计数的借用，而非拷贝
            self.0 += 1;
            Some(&mut self.0)
        }
    }
    let mut it = Counter(0);
    let mut total = 0;
    for _ in 0..2 {
        // next() 借出 &mut usize，本轮借用结束后才能再次 next
        if let Some(x) = it.next() {
            *x += 5;
            total = *x;
        }
    }
    println!("  计数被就地修改为: {}", total); // 12（next 自身 +1，每次再 +5）
    // 注意：GATs 稳定后即可用；where 子句约束关联类型所需的生命周期关系。

    // ============================================================
    println!("\n3. label-break-value 从任意带标签块中 break");
    // 1.65 稳定：可以给普通块加标签并直接 break 出去，无需再写循环。
    let x = 42;
    let out = 'blk: {
        if x > 40 {
            break 'blk "big"; // 直接跳出块并作为块表达式值
        }
        "small"
    };
    println!("  块表达式的值 = {}", out);
    // 早期 break-with-value 需要包一层循环；现在 `'label: { ... break 'label val; }` 即可。

    // ============================================================
    println!("\n4. Stabilized APIs");
    // 4.1 std::backtrace::Backtrace —— 捕获栈回溯
    let bt = std::backtrace::Backtrace::capture();
    println!("  Backtrace::capture() 状态 = {:?}", bt.status());
    // 4.2 Bound::as_ref
    let b: std::ops::Bound<i32> = std::ops::Bound::Included(5);
    println!("  Bound::as_ref -> {:?}", b.as_ref()); // Included(&5)
    // 4.3 std::io::read_to_string —— 自由函数稳定
    let s = std::io::read_to_string(std::io::Cursor::new(b"hello 1.65")).unwrap();
    println!("  read_to_string(Cursor) -> {:?}", s);
    // 4.4/4.5 指针 cast_mut / cast_const（1.64 曾短暂可用，1.65 正式稳定）
    let cp: *const i32 = &5;
    let mp: *mut i32 = cp.cast_mut();
    let back: *const i32 = mp.cast_const();
    println!("  cast_mut -> cast_const 后读取 = {}", unsafe { *back });

    // ============================================================
    println!("\n5. MaybeUninit 与未初始化内存");
    // 1.65 起：读未初始化的整数/浮点/裸指针是 immediate UB。
    // 官方指引：操作未初始化内存必须用 MaybeUninit。
    let mut buf: [u8; 8] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    // 写入值，全初始化后可正常使用
    buf.copy_from_slice(b"1.65.0!\0");
    println!("  MaybeUninit::zeroed 初始化的缓冲 = {:?}", std::str::from_utf8(&buf));
    // 错误做法（UB，勿用）：let x: i32; let y = x + 1; // 读未初始化整数 = immediate UB

    // ============================================================
    println!("\n6. 兼容性提醒（Compatibility Notes）");
    println!("  - PollFn 只在闭包 Unpin 时才实现 Unpin");
    println!("  - 单个重复生命周期不再参与返回类型省略（撤销 1.64 的误改）");
    println!("  - deny #![cfg_attr(..., crate_type = ...)]");
}
