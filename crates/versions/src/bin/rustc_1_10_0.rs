// rustc 1.10.0 演示 —— panic hook / compare_exchange / CStr / 二分查找
use std::ffi::CStr;
use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("rustc 1.10.0 演示");

    println!("\n1. panic::set_hook / take_hook / PanicInfo::location（1.10 稳定）");
    // 换上自定义 panic 钩子，捕获 PanicInfo 的位置信息，之后恢复默认钩子
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(|info| {
        if let Some(loc) = info.location() {
            println!("   [hook] panic 位于 {}:{}（payload 动态获取略）", loc.file(), loc.line());
        }
    }));
    let _ = panic::catch_unwind(|| panic!("hook 触发"));
    panic::set_hook(default_hook);
    println!("钩子已恢复默认");

    println!("\n2. AtomicUsize::compare_exchange（1.10 稳定）");
    let a = AtomicUsize::new(10);
    // 期望 10，成功换成 20；CompareAndSwap 的"带回旧值"版本
    let r = a.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);
    println!("compare_exchange(10->20) = {:?}，现值 = {}", r, a.load(Ordering::SeqCst));
    let r2 = a.compare_exchange(10, 30, Ordering::SeqCst, Ordering::SeqCst);
    println!("compare_exchange(10->30) 失败返回旧值 {:?}", r2);

    println!("\n3. CStr::from_bytes_with_nul（1.10 稳定）");
    let cs = CStr::from_bytes_with_nul(b"safe\0");
    match cs {
        Ok(s) => println!("合法以 NUL 结尾的字节串 -> {:?}", s),
        Err(e) => println!("FromBytesWithNulError: {}", e),
    }
    println!("缺 NUL 会报错：{:?}", CStr::from_bytes_with_nul(b"bad").is_err());

    println!("\n4. binary_search_by_key");
    let v = [1, 3, 5, 7, 9];
    println!("按 x%%10 找 7 -> {:?}", v.binary_search_by_key(&7, |x| x % 10));

    println!("\n5. Weak::new（1.10 稳定）");
    let weak: std::sync::Weak<usize> = std::sync::Weak::new();
    println!("空 Weak upgrade = {:?}", weak.upgrade());

    println!("\n6. panic=abort / cdylib（println 讲解节）");
    // 当年形态：-C panic=abort 让 panic 直接中止进程（RFC 1513）；
    // 新 crate 类型 cdylib 用于生成给 C/其他语言加载的动态库（RFC 1510）。
    // 两者均为编译/构建配置，无法在单文件演示里直接体现。
    println!("-C panic=abort：panic 不再展开栈而是 abort；cdylib：跨语言动态库目标。");
}
