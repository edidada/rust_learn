// rustc 1.40.0 演示 —— non_exhaustive / mem::take / todo! / Option 工具 / 浮点字节序 API
use std::collections::HashMap;
use std::mem;

// 1.40.0 稳定 #[non_exhaustive]：跨 crate 使用方必须留通配分支、不能以函数式构造该结构。
// （这里模拟"另一个 crate 的类型"在同一 crate 内演示带通配分支的 match）
#[non_exhaustive]
pub enum Action {
    Start,
    Stop,
}

// 1.40.0 稳定 todo! —— 语义化"还没写完"：编译通过、运行时 panic 并提示 not yet implemented。
#[allow(dead_code)]
fn unfinished(x: u8) -> u8 {
    todo!("finish the algorithm")
}

fn main() {
    println!("rustc 1.40.0 演示");

    // 1.40.0 引入：#[non_exhaustive] —— 作者以后可以不破坏版本就给枚举加变体；
    // 使用方写 match 时必须显式 `_ =>` 分支。
    println!("\n1. #[non_exhaustive]");
    for a in [Action::Start, Action::Stop] {
        let text = match a {
            Action::Start => String::from("启动"),
            Action::Stop => String::from("停止"),
            _ => String::from("(未来新增的变体)"), // 必须；删掉这行编译错误
        };
        println!("match 结果: {}", text);
    }
    // 说明：unfinished 实际调用会 panic：'not yet implemented: finish the algorithm'
    println!("todo! 编译 OK（运行真正的调用才会 panic，此处未调用）");

    // 1.40.0 稳定 mem::take —— 等价 mem::replace(t, T::default())，把值拿走、留下默认值。
    println!("\n2. mem::take");
    let mut buf = String::from("我要被拿走");
    let got = mem::take(&mut buf);
    println!("take 后 buf = {:?}，拿到的 = {:?}", buf, got);

    // 1.40.0 稳定：Option::as_deref / Option::flatten。
    println!("\n3. Option::{as_deref, flatten}");
    let owned: Option<String> = Some(String::from("borrow me"));
    let borrowed: Option<&str> = owned.as_deref(); // Option<String>（调 Deref）→ Option<&str>
    println!("Some(String).as_deref() = {:?}", borrowed);
    let nested: Option<Option<u8>> = Some(Some(3));
    println!("Some(Some(3)).flatten() = {:?}", nested.flatten());
    println!("None::<Option<u8>>.flatten() = {:?}", None::<Option<u8>>.flatten());

    // 1.40.0 稳定：f32/f64 的 to/from_be_bytes、to_le_bytes 等。
    println!("\n4. f32/f64 <-> bytes");
    let f = 3.5f32;
    println!("3.5f32.to_be_bytes() = {:?}", f.to_be_bytes());
    let g = f64::from_be_bytes([64u8, 12, 0, 0, 0, 0, 0, 0]); // 3.5 的 f64 大端表示
    println!("from_be_bytes([64,12,0,0,0,0,0,0]) = {}", g);

    // 1.40.0 稳定：slice::repeat、HashMap::get_key_value。
    println!("\n5. slice::repeat / get_key_value");
    let base = [1u8, 2];
    println!("[1,2].repeat(3) = {:?}", base.repeat(3));
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("a", 1);
    println!("HashMap::get_key_value(&\"a\") = {:?}", m.get_key_value("a"));
}
