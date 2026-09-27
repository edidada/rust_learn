// rustc 1.58.0 演示 —— inline format args、is_symlink、saturating_div
fn main() {
    println!("rustc 1.58.0 演示");

    // ============================================================
    println!("\n1. inline format args：格式串直接捕获标识符");
    // 1.58 稳定：{ident} 直接引用同名变量；对比旧写法 format!("{}, v", v)。
    let name = "rust";
    let version = 58;
    println!("  hello, {name}!");
    println!("  rustc {version}");
    // 表达式也可（简单字段/路径级），如 {x.y}
    struct P {
        x: u32,
    }
    let p = P { x: 7 };
    println!("  字段捕获 p.x = {}", p.x);
    // 适合 write!/print! 系所有宏。（panic!("{ident}") 需 2021 edition）

    // ============================================================
    println!("\n2. Path::is_symlink 与 File::options");
    use std::path::Path;
    println!("  Path::new('nope').is_symlink() = {}", Path::new("nope").is_symlink());
    // File::options：OpenOptions::new() 的等价快捷方法
    use std::fs::File;
    // OpenOptions::new 与 File::options 是等价入口（不真正打开文件，仅演示构造链）。
    let _opts = File::options().read(true).create(false);
    println!("  File::options().read(true) 构造成功 = true");

    // ============================================================
    println!("\n3. {integer}::saturating_div");
    // 1.58 稳定：除法溢出（i32::MIN / -1）时饱和成 i32::MAX 而不 panic。
    println!("  (i32::MIN).saturating_div(-1) = {}", i32::MIN.saturating_div(-1));
    println!("  7.saturating_div(2) = {}", 7.saturating_div(2));
    // 普通除法此情形是 panic:（println 讲解，不真除零）

    // ============================================================
    println!("\n4. Duration 在 const 上下文可用");
    use std::time::Duration;
    // 1.58 起 Duration::new 与 checked/饱和算术在 const 上下文可用
    const D1: Duration = Duration::new(2, 0);
    const D2: Duration = Duration::from_secs(1);
    const D3: Duration = match D1.checked_add(D2) {
        Some(d) => d,
        None => Duration::ZERO,
    };
    println!("  const D3 (2s + 1s) = {:?}", D3);

    // ============================================================
    println!("\n5. 兼容性提醒");
    println!("  - Windows 上 process::Command 不再搜索当前目录（CVE 相关安全性）。");
    println!("  - proc-macro 后向兼容 lint 全部 deny-by-default。");
    println!("  - std::sys::unix 弱符号重构（弱链接替代 dlopen）。");
}
