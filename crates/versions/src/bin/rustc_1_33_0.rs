// rustc 1.33.0 演示 —— 整数穷尽匹配 + if let 多模式 + use Trait as _
// 该版本 introduces:
// 1) 整数模式可穷尽：match u8 覆盖 0..=255 后不需要 _ 兜底（此前必须）。
// 2) if let / while let 支持多模式 |；不可反驳模式（默认告警，略演示）。
// 3) use Trait as _：只带 impl 不占名。
// 4) trim_left/right 正式弃用 -> trim_start/trim_end（1.30 已加新名）。
// 5) Option::transpose / convert::identity / Vec::resize_with / repr(packed(N))。

fn main() {
    println!("rustc 1.33.0 演示");

    // 1. 整数穷尽匹配：u8 全覆盖，无 _ 分支
    println!("\n1. 整数模式穷尽");
    let n: u8 = 200;
    let bucket = match n {
        0..=127 => "低半区",
        128..=255 => "高半区",
        // 1.33 前：编译器认为还需要 _ => unreachable!()
    };
    println!("{} 属于 {}", n, bucket);

    // 2. if let 多模式
    println!("\n2. if let 多模式 |");
    enum Creature {
        Crab(String),
        Lobster(String),
        Person(String),
    }
    let state = Creature::Crab("Ferris".to_string());
    if let Creature::Crab(name) | Creature::Person(name) = state {
        println!("这个生物叫 {}", name);
    }

    // 3. use Trait as _
    println!("\n3. use Trait as _");
    use std::fmt::Write as _; // 只启用 impl，不引入名字 Write
    let mut buf = String::new();
    write!(&mut buf, "as _ 导入的 Write 写入 {}", 42).unwrap();
    println!("{}", buf);

    // 4. trim_start / trim_end（对照弃用的 trim_left/trim_right）
    println!("\n4. trim_start / trim_end");
    let s = "--rust--";
    println!("trim_start_matches('-') = \"{}\"", s.trim_start_matches('-')); // 旧 trim_left_matches
    println!("trim_end_matches('-')   = \"{}\"", s.trim_end_matches('-'));   // 旧 trim_right_matches

    // 5. Option::transpose / convert::identity
    println!("\n5. transpose / identity");
    let opt_res: Option<Result<i32, ()>> = Some(Ok(5));
    println!("Option(Ok(5)).transpose() = {:?}", opt_res.transpose());
    let id = std::convert::identity::<i32>(7);
    println!("identity(7) = {}", id);

    // 6. Vec::resize_with / Duration::as_millis
    println!("\n6. resize_with / as_millis");
    let mut v: Vec<u32> = Vec::new();
    v.resize_with(3, || 7);
    println!("resize_with(3, ||7) = {:?}", v);
    let d = std::time::Duration::from_millis(2500);
    println!("2500ms -> as_millis = {}", d.as_millis());

    // 7. #[repr(packed(2))]
    println!("\n7. #[repr(packed(2))]");
    #[repr(packed(2))]
    struct Packed(i16, i32);
    println!(
        "size_of::<Packed>() = {}（6 字节：2+4，对齐 2）",
        std::mem::size_of::<Packed>()
    );
}
