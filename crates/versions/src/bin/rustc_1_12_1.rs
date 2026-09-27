// rustc 1.12.1 演示 —— 1.12.0 回归修复验证（补丁版）
fn main() {
    println!("rustc 1.12.1 演示（补丁：修复 1.12.0 回归）");

    println!("\n1. 布尔双重取反（!!x）回归修复");
    // 1.12.0 曾让 !!bool 的求值出现混淆，1.12.1 修复；
    // 现在两个 NOT 叠加应完全还原原值
    let flag = true;
    let double_not = !!flag;
    println!("flag = {}, !!flag = {}, 还原一致 = {}", flag, double_not, flag == double_not);

    println!("\n2. 其余回归修复项（println 讲解节）");
    // 当年形态：1.12.1 只包含编译器回归修复，无新特性：
    // - ICE: concrete_substs.is_normalized_for_trans()
    // - release 模式 SIGSEGV（syn 0.8.0）/ ethcore Windows LLVM 错误
    // - 带调试信息的 release 链接内存占用过高
    // - 1.12 升级后的内存损坏
    // - `Let NullaryConstructor = ...;` 的 ICE
    // - invoke/call/store 类型不匹配时注入 bitcast
    // - debuginfo spread_arg 的稳定处理
    println!("以上均为编译器层修复，无法在单文件演示中复现，仅列出条目。");
}
