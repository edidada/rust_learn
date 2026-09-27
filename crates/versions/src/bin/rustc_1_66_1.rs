// rustc 1.66.1 演示 —— patch 版本：Cargo SSH host key 校验（CVE-2022-46176）
// 这是 Cargo 侧的安全修复，无法在普通代码里运行演示，仅以文字说明。
fn main() {
    println!("rustc 1.66.1 演示");
    println!("\n1. CVE-2022-46176：Cargo git 依赖的 SSH host key 校验");
    println!("  - 修复前：Cargo 通过 SSH 协议拉取 git 依赖时不校验 SSH 主机密钥，");
    println!("    中间人（MITM）攻击者可冒充 git 服务器，截获凭据或注入恶意代码。");
    println!("  - 修复后：Cargo 增加 SSH host key 校验，遇到未知主机密钥将拒绝连接。");
    println!("  - 该修复属于构建工具行为变更，对普通源码无 API 影响。");
}
