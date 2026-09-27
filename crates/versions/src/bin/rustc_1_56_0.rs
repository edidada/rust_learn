// rustc 1.56.0 演示 —— Edition 2021：闭包分离捕获、数组 into_iter、panic 一致性
fn main() {
    println!("rustc 1.56.0 演示");

    // ============================================================
    println!("\n1. Edition 2021 行为差异：闭包分离捕获（disjoint capture）");
    // 2015/2018：闭包借用整个结构体；2021：只闭包用到哪个字段就捕获哪个字段。
    // 效果：一个字段被闭包可变借用时，其它字段仍可被外部同时借用/写入。
    struct Counters {
        hit: u32,
        miss: u32,
    }
    let mut c = Counters { hit: 0, miss: 0 };
    // 只捕获 c.hit，因此循环里还能摸 c.miss
    let mut bump_hit = || c.hit += 1;
    for i in 0..3 {
        c.miss = i; // 2021 下合法；2018 下与 bump_hit 的整体借用冲突
        bump_hit();
    }
    bump_hit();
    println!("  hit = {}, miss = {}", c.hit, c.miss);

    // ============================================================
    println!("\n2. [T; N].into_iter() 在 2021 edition 按值迭代");
    // 1.53 引入 IntoIterator for [T; N]：当时 arr.into_iter() 借出 &T 且会警告；
    // Edition 2021 起 .into_iter() 按值移动元素 —— 历史语义正式落地。
    let arr = [1u8, 2, 3];
    let mut s = 0;
    for v in arr.into_iter() {
        s += v; // v: u8（按值）
    }
    println!("  [1,2,3].into_iter() 累计 = {}", s);
    // 泛型 fn 一视同仁接收数组/Vec：
    fn total<I: IntoIterator<Item = u8>>(it: I) -> u8 {
        it.into_iter().sum()
    }
    println!("  total(数组) = {}, total(vec) = {}", total(arr), total(vec![9, 9]));

    // ============================================================
    println!("\n3. panic 宏一致性（println 讲解，不真是 panic）");
    println!("  - 2021 起 panic!(\"非格式化字符串参数\") 不再隐式当显示文本；");
    println!("    需 panic!(\"{{}}\", x) 或 std::panic::panic_any(x)。");
    println!("  - panic!(\"{{ident}}\") 捕获变量要等 1.58（需 2021 edition）。");

    // ============================================================
    println!("\n4. 集合 From<[(K, V); N]>");
    use std::collections::HashMap;
    // 1.56 稳定：字面数组一步变集合
    let m: HashMap<&str, i32> = HashMap::from([("a", 1), ("b", 2)]);
    println!("  HashMap::from([(a,1),(b,2)]) 里的 a = {:?}", m.get("a"));

    // ============================================================
    println!("\n5. TryFrom / TryInto 在 edition 2021 的 prelude");
    // 2021 起 TryFrom/TryInto/FromIterator 无需 use std::convert::* 即可用。
    let small: Result<u8, _> = u8::try_from(200i32);
    println!("  u8::try_from(200) = {:?}", small.ok());
    let too_big: Result<u8, _> = u8::try_from(300i32);
    println!("  u8::try_from(300) 错误 = {:?}", too_big.err().map(|e| e.to_string()));

    // ============================================================
    println!("\n6. 环境变量名含 null / '=' 按不存在处理");
    // 1.56 前：这类名字会 panic；之后：当作“该变量不存在”。
    let weird = std::env::var("we\0ird");
    println!("  env::var(含 \\0 名字) -> Err({:?})", weird.is_err());

    // ============================================================
    println!("\n7. 兼容性提醒");
    println!("  - Windows 命令行参数解析规则对齐 C/C++（2008 后最新）。");
    println!("  - aarch64 禁 aapcs 调用约定。");
    println!("  - SEMICOLON_IN_EXPRESSIONS_FROM_MACROS 默认 warn。");
}
