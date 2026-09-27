// rustc 1.4.0 演示 —— MSVC 支持 / use 重命名 / parse 往返 / Rc/Arc API
mod foo {
    pub fn bar() -> &'static str {
        "kitten"
    }
    pub fn baz() -> &'static str {
        "puppy"
    }
}

use foo::{bar as kitten, baz as puppy};

fn main() {
    println!("rustc 1.4.0 演示");

    println!("\n1. use 重命名多项导入（1.4 新语法）");
    // 当年新引入：use foo::{bar as kitten, baz as puppy}（foo 定义在文件顶部）
    println!("bar as kitten -> {}, baz as puppy -> {}", kitten(), puppy());

    println!("\n2. lines 把 \\r\\n 也视为换行（1.4 行为变化）");
    let lines: Vec<&str> = "a\r\nb\nc".lines().collect();
    println!("{:?}", lines);

    println!("\n3. str::parse 不再有可避免的舍入误差；接受前导 +");
    let f = 0.1_f64;
    // 1.4 起 f.to_string().parse::<f64>() == f 的精确往返成立
    let back: f64 = f.to_string().parse().unwrap();
    println!("{} -> parse -> {}（相等：{}）", f, back, f == back);
    let positive: i32 = "+42".parse().unwrap();
    println!("\"+42\".parse() = {}", positive);

    println!("\n4. Rc::try_unwrap / Arc::get_mut 稳定化");
    use std::rc::Rc;
    use std::sync::Arc;
    let rc = Rc::new(7_i32);
    match Rc::try_unwrap(rc) {
        Ok(v) => println!("try_unwrap 拿到独占值 {}", v),
        Err(_) => println!("仍有共享者，无法解包"),
    }
    let mut arc = Arc::new(10_i32);
    *Arc::get_mut(&mut arc).unwrap() += 5; // 只有一个所有者时可变
    println!("Arc::get_mut 后 = {}", *arc);

    println!("\n5. &Option 与 &Result 实现 IntoIterator");
    let o: Option<i32> = Some(3);
    let r: Result<i32, &str> = Ok(9);
    println!("&Option -> {:?}, &Result -> {:?}", o.iter().collect::<Vec<_>>(), r.iter().collect::<Vec<i32>>());

    println!("\n6. CStr::to_str / to_string_lossy 稳定化");
    use std::ffi::CString;
    let cs = CString::new("weird \u{fffd} text").unwrap();
    println!("to_string_lossy = {:?}", cs.to_string_lossy());

    println!("\n7. Result::expect（带 msg 的 unwrap）");
    let ok: Result<i32, &str> = Ok(5);
    println!("expect:: {}", ok.expect("should be ok"));
}
