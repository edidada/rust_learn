// rustc 1.94.1 演示 —— patch：wasip1-threads spawn、OpenOptionsExt 非默认方法回退、clippy ICE、tar CVE（记叙）
use std::fs::OpenOptions;

fn main() {
    println!("rustc 1.94.1 演示（patch 版）");

    println!("\n1. wasm32-wasip1-threads 线程 spawn 修复");
    println!("修复该目标上 std::thread::spawn 不可用/异常的问题（平台后端修）");

    println!("\n2. std::os::windows::fs::OpenOptionsExt 新方法回退 —— trait 扩展规则记叙");
    // trait 未 sealed 时向其加非 default 方法 = 破坏下游实现者；
    // 所以"未 seal 的 trait"新增方法必须保持 default 或回退，等 trait sealed 后再补
    let _oo: OpenOptions = OpenOptions::new();
    println!("OpenOptionsExt 未 sealed → 新加的非 default 方法被移除（1.94.1 回退）");

    println!("\n3. clippy match_same_arms ICE 修复");
    println!("仅 clippy 侧修复，rustc 无行为变化");

    println!("\n4. Cargo tar 0.4.45（CVE-2026-33055/33056）");
    println!("打包/解包路径相关 CVE 修复；crates.io 用户不受影响");
}
