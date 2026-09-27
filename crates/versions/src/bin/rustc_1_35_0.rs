// rustc 1.35.0 演示 —— Box<F> 可直接调用 / 闭包转 unsafe fn / dbg!() / Range::contains
fn main() {
    println!("rustc 1.35.0 演示");
    // 1.35.0 引入：FnOnce/FnMut/Fn 这三个闭包 trait 现在对 Box<dyn Fn*> 实现了，
    // 所以 Box<dyn FnOnce> 可以像普通函数一样用 `()` 直接调用。
    println!("\n1. Box<dyn FnOnce/...> 直接调用");
    let f: Box<dyn FnOnce() -> i32> = Box::new(|| 1 + 2);
    println!("Box<dyn FnOnce()>() = {}", f()); // 旧版要写 (*f)()
    let mut count = 0;
    let mut g: Box<dyn FnMut()> = Box::new(|| count += 1);
    g();
    g();
    println!("Box<dyn FnMut()>() 调用两次后 count = {}", count);
    let h: Box<dyn Fn() -> &str> = Box::new(|| "hello from boxed closure");
    println!("Box<dyn Fn()>() = {}", h());

    // 1.35.0 引入：闭包可以直接强制转换（coerce）为 unsafe fn 指针，
    // 前提是闭包不捕获环境（non-capturing closure）。
    println!("\n2. 闭包 coerce 为 unsafe fn");
    struct Registrar {
        hooks: Vec<unsafe fn()>,
    }
    // 闭包 `|| log_hook()` 是无捕获闭包，可以直接填 unsafe fn() 的位置
    unsafe fn log_hook() {
        println!("[hook called]");
    }
    let reg = Registrar {
        hooks: vec![log_hook, || log_hook()], // 第二个就是闭包强转成 unsafe fn
    };
    unsafe {
        for hook in &reg.hooks {
            hook();
        }
    }

    // 1.35.0 引入：dbg!() 无参数调用，打印调用处的 文件:行号 再返回值。
    println!("\n3. dbg!() 无参用法");
    let value = dbg!(); // 打印类似 [src\bin\rustc_1_35_0.rs:行:列] 里的信息
    println!("dbg!() 返回 ()，且已打印当前位置信息；value = {:?}", value);
    let n = dbg!(2 + 3); // 对照：带参数的 dbg! 会打印表达式与值
    println!("带参 dbg!(2+3) = {}（并额外打印该求值）", n);

    // 1.35.0 稳定：RangeXxx::contains 系列，不再需要写 `r.start <= x && x < r.end`。
    println!("\n4. Range::contains 系列");
    let r = 0..10;
    println!("(0..10).contains(&5) = {}", r.contains(&5));
    println!("(0..10).contains(&10) = {}", r.contains(&10));
    let rf = 0..;
    println!("(0..).contains(&10000) = {}", rf.contains(&10000));
    let rt = ..10;
    println!("(..10).contains(&3) = {}", rt.contains(&3));
    let ri = 0..=10;
    println!("(0..=10).contains(&10) = {}", ri.contains(&10));
    let rti = ..=10;
    println!("(..=10).contains(&10) = {}", rti.contains(&10));

    // 1.35.0 稳定：Option::copied —— 把 Option<&T> 映射成 Option<T>（要求 T: Copy）。
    println!("\n5. Option::copied");
    let opt_ref: Option<&u8> = Some(&7);
    println!("Some(&7).copied() = {:?}", opt_ref.copied());
    let none_ref: Option<&u8> = None;
    println!("None::<&u8>.copied() = {:?}", none_ref.copied());
}
