// rustc 1.25.0 演示 —— impl Trait 泛型参数位 + repr(align) + 嵌套导入组
// 该版本 introduces:
// 1) Language：泛型参数位置（argument position）的 impl Trait 稳定：
//      fn f(x: impl Trait) {}
//    对比旧行为：只能写 fn f<T: Trait>(x: T)，"(impl Trait)" 那时是 nightly-only。
//    注意：返回位置的 `-> impl Trait` 要到 1.26 才稳定，本文件不演示。
// 2) #[repr(align(N))]：手动抬高结构体的对齐。
// 3) 嵌套导入组、match 分支行首 `|`、ptr::NonNull、Duration 的 const fn。

// 3. 嵌套导入组（1.25 稳定）：同一括号里还能继续嵌套 {}
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

enum Cmd {
    Start,
    Stop,
    Pause,
}

// —— 1. 泛型参数位置的 impl Trait ——
fn sum_of(a: impl Into<u64>, b: impl Into<u64>) -> u64 {
    a.into() + b.into()
}

// 对比：老写法是显式泛型参数
fn sum_of_old<T: Into<u64>>(a: T, b: T) -> u64 {
    a.into() + b.into()
}

fn show(x: impl Display) {
    println!("impl Display 参数: {}", x);
}

fn main() {
    println!("rustc 1.25.0 演示");

    // 1. 泛型参数位置的 impl Trait
    println!("\n1. impl Trait 在参数位置");
    println!("sum_of(3u8,4u16)  = {}", sum_of(3u8, 4u16));
    println!("sum_of_old::<u8>() = {}", sum_of_old(3u8, 4u8));
    show(12345);

    // 2. #[repr(align(16))]：对齐从 4 抬到 16
    println!("\n2. #[repr(align(16))]");
    #[repr(align(16))]
    struct Simd {
        vals: [f32; 4], // 本来只对齐 4 字节
    }
    println!(
        "align_of::<Simd>() = {}, size_of::<Simd>() = {}",
        std::mem::align_of::<Simd>(),
        std::mem::size_of::<Simd>()
    );

    // 3. 嵌套导入组已在顶部 use 中演示
    println!("\n3. 嵌套导入组");
    let p = PathBuf::from("demo.txt");
    println!("Path().is_file 之类的抽象演示：PathBuf = {}", p.display());
    let _has_path_marker = true;
    if _has_path_marker {
        fn _t(_: &Path) {}
        let _ = _t;
    }

    // 4. match 分支的行首 |
    println!("\n4. 行首 | 的 match 分支");
    let c = Cmd::Stop;
    match c {
        | Cmd::Start
        | Cmd::Stop => println!("Start 或 Stop"),
        | Cmd::Pause => println!("Pause"),
    }

    // 5. ptr::NonNull：非空指针抽象（1.25 稳定）
    println!("\n5. ptr::NonNull");
    let boxed = Box::new(42i32);
    let ptr = Box::into_raw(boxed);
    if let Some(nn) = std::ptr::NonNull::new(ptr) {
        println!("NonNull 保存值 = {}", unsafe { nn.as_ref() });
        unsafe { drop(Box::from_raw(nn.as_ptr())) };
    }

    // 6. Duration::from_secs 等 const fn 化
    println!("\n6. Duration const-fn 化");
    const MINUTE: std::time::Duration = std::time::Duration::from_secs(60);
    println!("const MINUTE = Duration::from_secs(60) -> {}", MINUTE.as_secs());

    // 提示：Location::column 也在本版稳定（数据来自 panic hook 的 PanicInfo），
    // 无稳定 API 可直接构造 Location，故仅 println 讲解。
    println!("\n7. Location::column（列号 1 起，配合 panic 信息来源定位）— 讲解性输出");
}
