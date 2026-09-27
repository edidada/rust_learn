// rustc 1.99.0 演示 —— beta 版记叙（页面尚未定稿；工具链 1.98，不调用任何 1.99 新 API）
fn main() {
    println!("rustc 1.99.0 演示（beta 未发布，仅记叙）");

    println!("\n1. c-variadic 函数定义稳定路线");
    println!("1.91 稳定 sysv64/win64/efiapi/aapcs 的*声明*；1.93 稳定 system ABI 声明；1.99 稳定*定义*（c_variadic_naked_functions 路线）");
    println!("历史形态：unsafe extern \"C\" {{ fn printf(fmt: *const c_char, ...) -> i32; }} 只能声明不能定义");

    println!("\n2. 库侧稳定项（记叙）");
    println!("fs_set_times 稳定（File::set_modified/set_accessed 之类时间 API 家族）");
    println!("String::from_utf8_lossy_owned 稳定：直接消费 Vec<u8> 出 String，不复制");
    println!("PinSafePointer 泛化 PinCoerceUnsized");

    println!("\n3. UnsafeCell 直访 + diagnostic::opaque");
    println!("允许不经过 get() 访问 UnsafeCell 内容（invalid_reference_casting lint 放宽）");
    println!("#[diagnostic::opaque]：宏展开报错时隐藏宏 backtrace");

    println!("\n4. 编译器侧（记叙）");
    println!("LLVM 23；-Ctarget-cpu 变 target-modifier（AVR/AMDCGN/NVPTX）；POSIX 符号 lint 扩展");
    println!("riscv64-unknown-linux-musl 升 Tier 2");
}
