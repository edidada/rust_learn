// rustc 1.77.2 演示 —— patch 版本：CVE-2024-24576 Windows 批处理参数转义（文字说明）
fn main() {
    println!("rustc 1.77.2 演示");
    println!("\n1. CVE-2024-24576（BatBadBut）：Windows 批处理文件参数转义");
    println!("  - std::process::Command 在 Windows 上运行 .bat/.cmd 时，");
    println!("    对含特殊字符的参数转义不充分，攻击者可注入任意命令；");
    println!("  - 1.77.2 修复该转义逻辑。普通非批处理调用不受影响。");
    println!("  - 安全修复无 API 变化，仅命令行行为演示（文字说明）。");
}
