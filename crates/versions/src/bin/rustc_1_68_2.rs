// rustc 1.68.2 演示 —— patch 版本：Cargo GitHub RSA host key 轮换（文字说明）
fn main() {
    println!("rustc 1.68.2 演示");
    println!("\n1. Cargo SSH host key 安全更新");
    println!("  - GitHub 于 2023-03-24 轮换 RSA host key（旧密钥泄露），Cargo 更新内置密钥；");
    println!("  - 旧密钥被标记为 @revoked，即使系统仍信任它，Cargo 也拒绝接受；");
    println!("  - SSH host key 校验支持 @revoked，并改进 @cert-authority 错误信息。");
}
