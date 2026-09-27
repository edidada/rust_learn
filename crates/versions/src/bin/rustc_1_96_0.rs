// rustc 1.96.0 演示 —— assert_matches!、LazyLock/Cell From<T>、core::range 系列、NonZero range 迭代
// 注意：本仓库以 rustc 1.98 运行；1.96 稳定的 API 在 1.98 均可用。
use core::assert_matches;
use core::debug_assert_matches;
use std::cell::LazyCell;
use std::num::NonZero;
use std::sync::LazyLock;

fn main() {
    println!("rustc 1.96.0 演示");

    println!("\n1. assert_matches! / debug_assert_matches!（1.96 稳定）");
    // 旧写法：match 表达式 + 手写 _ => panic!(...)；断言宏一步完成（1.96 经 prelude 直接可用）
    let a = Some(345);
    assert_matches!(a, Some(x) if x > 100);
    println!("assert_matches!(Some(345), Some(x) if x>100) 通过");
    let result: Result<i32, String> = Err("boom".into());
    assert_matches!(result, Err(msg) if msg.contains("oo"), "失败信息: {:?}", result);
    println!("带错误消息的 assert_matches! 通过");
    // debug_assert_matches! 只在 debug 构建里检查（release 为 no-op）
    debug_assert_matches!(a, Some(_));
    println!("debug_assert_matches!(a, Some(_)) 通过");

    println!("\n2. From<T> for LazyCell/LazyLock（1.96：从值直接构造已就绪的懒对象）");
    // 旧写法：LazyLock::new(|| value) 用闭包包一层；From<T> 表达"值已就绪"更直白
    let l: LazyLock<u32> = LazyLock::from(42);
    println!("LazyLock::from(42) → {}", *l);
    let c: LazyCell<String> = LazyCell::from("ready".to_string());
    println!("LazyCell::from(String) → {}", *c);

    println!("\n3. core::range 新 Range API（1.95/1.96 稳定齐套）");
    use std::range;
    // core::range::Range 是 Copy 的"值域"，从旧 Range::from(..) 转换
    let r: range::Range<i32> = range::Range::from(3..6);
    println!("range::Range::from(3..6).iter().sum() = {}", r.iter().sum::<i32>()); // 12
    let ri: range::RangeInclusive<i32> = range::RangeInclusive::from(1..=3);
    println!("RangeInclusive::from(1..=3) 迭代 = {:?}", ri.iter().collect::<Vec<i32>>());
    let rf: range::RangeFrom<i32> = range::RangeFrom::from(2..);
    println!("RangeFrom::from(2..) 取前 3 个 = {:?}",
        rf.iter().take(3).collect::<Vec<i32>>());
    let rt: range::RangeToInclusive<i32> = range::RangeToInclusive::from(..=2);
    println!("RangeToInclusive::from(..=2) 存在（配合上下文用于切片等）");
    println!("RangeToInclusive = {rt:?}");

    println!("\n4. NonZero 整数 range 迭代（1.96）");
    // 1.95 起 NonZero 实现 Step；本版把 NonZero range 的迭代与 range API 打通
    let nz: std::ops::RangeInclusive<NonZero<u32>> =
        NonZero::new(1).unwrap()..=NonZero::new(3).unwrap();
    println!("NonZero(1)..=NonZero(3) = {:?}", nz.clone().collect::<Vec<NonZero<u32>>>());
    println!("各元素取值 = {:?}",
        nz.map(|n| n.get()).collect::<Vec<u32>>());

    println!("\n5. 其余 1.96 兼容性条目（记叙）");
    println!("- 禁止 unsize 强转到 Pin<Foo>（Foo 不实现 Deref）");
    println!("- export_name/link_name/link_section 重复属性：第一个生效（旧行为相反）");
    println!("- BTreeMap::append() 优化：错误 Ord 实现可能 panic；c_double 在 avr 上改 f32");
    println!("- 移除 -Csoft-float；use S::{{self as Other}} 不再允许");
}
