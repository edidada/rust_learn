// arch 模块示例：SIMD和供应商内部函数模块

fn main() {
    // 1. 检查架构信息
    println!("1. Architecture information:");
    
    #[cfg(target_arch = "x86")]
    println!("Running on x86 architecture");
    
    #[cfg(target_arch = "x86_64")]
    println!("Running on x86_64 architecture");
    
    #[cfg(target_arch = "arm")]
    println!("Running on ARM architecture");
    
    #[cfg(target_arch = "aarch64")]
    println!("Running on AArch64 architecture");
    
    // 2. 使用架构特定的功能
    println!("\n2. Architecture-specific features:");
    
    // 注意：SIMD功能需要 nightly 版本和相应的特性标志
    // 以下代码仅作为示例，可能需要适当的配置才能编译
    
    // #[cfg(target_arch = "x86_64")]
    // unsafe {
    //     // 使用SSE指令
    //     use std::arch::x86_64::*;
    //     
    //     let a = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
    //     let b = _mm_set_ps(5.0, 6.0, 7.0, 8.0);
    //     let c = _mm_add_ps(a, b);
    //     
    //     println!("SIMD addition performed");
    // }
    
    println!("Arch module examples");
}