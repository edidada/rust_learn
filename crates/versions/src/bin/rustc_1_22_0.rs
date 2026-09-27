// rustc 1.22.0 演示 —— T op= &T + Option 上的 ?
// 该版本 introduces:
// 1) Language：复合赋值运算符支持右操作数为引用（x += &8），此前必须写成 x += *r。
// 2) Language：Option 实现了 Try trait，? 双向可用（1.22 前 ? 只能用于 Result）。
// 3）Unicode 转义允许用 '_' 分隔数字，长码点更好读。

fn main() {
    println!("rustc 1.22.0 演示");

    // 1. T op= &T：数字类型与引用做复合赋值
    println!("\n1. T op= &T（引用右操作数）");
    let mut x = 2;
    let r = &8;
    x += r; // 1.22 起直接可用；旧行为（1.21 前）：必须显式解引用 x += *r;
    println!("x += &8 后 x = {}", x);
    let mut y = 10.5f64;
    let r2 = &0.5;
    y *= r2;
    println!("y *= &0.5 后 y = {}", y);

    // 2. Option 上的 ?：返回 Option 的函数中传播 None
    println!("\n2. Option 有人权了：? 也可用");
    fn parse_pair(a: Option<&str>, b: Option<&str>) -> Option<i32> {
        let ia = a?.parse::<i32>().ok()?;
        let ib = b?.parse::<i32>().ok()?;
        Some(ia + ib)
    }
    println!("Some(\"3\") + Some(\"4\") -> {:?}", parse_pair(Some("3"), Some("4")));
    println!("Some(\"3\") + None       -> {:?}", parse_pair(Some("3"), None));

    // 3. Unicode 转义中的下划线：分组让长码点可读
    println!("\n3. unicode 转义允许下划线");
    let smile = '\u{1_f_600}'; // 1.22 前：\u{1F600} 必须连写
    println!("\\u{{1_F_600}} = {}", smile);
}
