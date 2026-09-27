// rustc 1.49.0 演示 —— ref+move 混合绑定 / union 实现 Drop / 空枚举 cast /
// select_nth_unstable / const Poll / RangeInclusive 耗尽检查
use std::cmp::Ordering;
use std::mem::ManuallyDrop;
use std::task::Poll;

// 1.49.0 语言特性：union 可以实现 Drop（此前 union 禁止 impl Drop）。
#[allow(dead_code)]
union OnlyInt {
    n: u32,
}

impl Drop for OnlyInt {
    fn drop(&mut self) {
        // 读取 union 字段须 unsafe；这里只打印，不真正读取数据
        println!("  OnlyInt 被析构（union 实现了 Drop）");
    }
}

// 1.49.0：union 字段可以是 ManuallyDrop<T>。
#[allow(dead_code)]
union MaybeStr {
    md: ManuallyDrop<String>,
    raw: u64, // 另一个字段：字节全 0 视为"无字符串"，避免读出假引用
}

fn main() {
    println!("rustc 1.49.0 演示");

    // 1.49.0：模式绑定可"部分移动、部分借用"——ref 与移动混用。
    println!("\n1. 模式混合绑定（ref + move）");
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person { name: "Alice".to_string(), age: 20 };
    // `name` 从 person 移出，`age` 仅借用：
    let Person { name, ref age } = person;
    println!("name = {}（已移动），age = {}（借用，person 部分失效）", name, age);
    // age 仍可读，person.name 已被移走，person 整体不可再整体使用 —— 借用检查保证安全。
    println!("借用字段 age * 2 = {}", age * 2);

    // 1.49.0：union 实现 Drop + ManuallyDrop 字段。
    println!("\n2. union 实现 Drop / ManuallyDrop 字段");
    {
        let _u = OnlyInt { n: 42 };
        println!("  作用域结束时触发 union 的 Drop");
    } // 此处自动调用 OnlyInt::drop
      // ManuallyDrop<String> 字段：构造/读取都是 unsafe（String 非 Copy）。
    let mut s = MaybeStr { md: ManuallyDrop::new("hi".to_string()) };
    unsafe {
        // 仅在字段仍持有字符串时读取；读完立即手动释放，杜绝 union 的双重释放
        if (*s.md).len() > 0 {
            println!("  ManuallyDrop<String> 字段内容 = {:?}", &*s.md);
            ManuallyDrop::drop(&mut s.md);
        }
    }

    // 1.49.0：uninhabited（空）枚举可以 cast 成整数。
    println!("\n3. 空枚举 cast 为整数");
    enum Void {} // 无任何变体，不可能存在值
    // 1.49 起 `v as u32` 可编译；1.48 及之前会报错：
    fn never_to_int(v: Void) -> u32 {
        v as u32
    }
    // never_to_int 无法被实际调用（造不出 Void 值），仅证明可编译：
    println!("fn never_to_int(v: Void) -> u32 {{ v as u32 }} 可编译（此函数不会被调用）");
    let _ = never_to_int;

    // 1.49.0 稳定：slice::select_nth_unstable —— 原地把第 n 小元素放到正确位置。
    println!("\n4. slice::select_nth_unstable 系列");
    let mut nums = [5, 9, 1, 3, 7, 2, 8];
    let (left, nth, right) = nums.select_nth_unstable(2); // 第 2 索引 = 第 3 小
    println!("select_nth_unstable(2)：中位值 = {}（已归位）", nth);
    println!("小于它的 = {:?}，大于等于它的 = {:?}", left, right);
    let mut words = ["pear", "apple", "fig"];
    let (_, mid, _) = words.select_nth_unstable_by_key(0, |w| w.len());
    println!("by_key(len) 选最短：{}", mid);

    // 1.49.0：Poll::is_ready / is_pending 成为 const。
    println!("\n5. const：Poll::is_ready / is_pending");
    const RDY: bool = Poll::<i32>::Ready(1).is_ready();
    const PEND: bool = Poll::<i32>::Pending.is_pending();
    println!("const Ready(1).is_ready()  = {}", RDY);
    println!("const Pending.is_pending() = {}", PEND);

    // 1.49.0：RangeInclusive::contains 与索引现在会检查耗尽状态。
    println!("\n6. RangeInclusive 耗尽检查");
    let mut r = 0..=2;
    for _ in r.by_ref() {} // for 循环把 r 消耗到"已耗尽"状态
    // 耗尽后按逻辑应视为空区间：1.49 起 contains 返回 false（1.48 前误判为 true）
    println!("耗尽后的 0..=2：is_empty = {}，contains(&1) = {}",
        r.is_empty(), r.contains(&1));
    let fresh = 0..=2;
    println!("未耗尽的 0..=2：contains(&1) = {}（正常判断）", fresh.contains(&1));

    // 1.49.0 其他：测试线程输出被捕获、aarch64-unknown-linux-gnu 升 tier 1、
    // cargo-package 可复现、CARGO_PRIMARY_PACKAGE 变量、宏尾分号按语句处理 ——
    // 均为工具链/编译期行为，无运行期演示，详见 notes/1.49.0.md。
    println!("\n7. 其余条目为工具链行为（详见 notes/1.49.0.md）");
    println!("最低 LLVM 版本提至 9；i686-freebsd 降级；关联类型不再推断 trait 约束等");
    let _ = Ordering::Equal;
}
