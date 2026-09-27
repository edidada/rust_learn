// rustc 1.18.0 演示 —— pub(crate) / retain / try_wait / 字段重排
use std::collections::{HashMap, HashSet};

fn main() {
    println!("rustc 1.18.0 演示");

    println!("\n1. pub(crate) 与 pub(in path)（RFC 1422，1.18 稳定）");
    println!("helper::visible_in_crate() = {}", helper::visible_in_crate());

    println!("\n2. HashMap::retain / HashSet::retain（1.18 稳定）");
    let mut scores = HashMap::new();
    scores.insert("a", 90);
    scores.insert("b", 40);
    scores.insert("c", 70);
    scores.retain(|_, v| *v >= 60); // 原地按条件删除
    println!("retain(>=60) 后 = {:?}", scores);
    let mut set: HashSet<i32> = [1, 2, 3, 4].into();
    set.retain(|x| x % 2 == 0);
    println!("retain 偶数 = {:?}", set);

    println!("\n3. Child::try_wait（1.18 稳定，非阻塞等待）");
    // 当年形态：try_wait 首次稳定；轮询子进程而不挂起当前线程
    #[cfg(windows)]
    let mut child = std::process::Command::new("cmd").args(["/C", "exit 0"]).spawn().unwrap();
    #[cfg(not(windows))]
    let mut child = std::process::Command::new("true").spawn().unwrap();
    let mut attempts = 0;
    let status = loop {
        match child.try_wait().unwrap() {
            Some(st) => break st,
            None => {
                attempts += 1;
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }
    };
    println!("轮询 {} 次后子进程结束，success = {}", attempts, status.success());

    println!("\n4. 0e+10 合法浮点字面量（1.18 起）");
    let big: f64 = 0e+10;
    println!("0e+10 = {}", big);

    println!("\n5. 结构体字段重排减小 padding（println 讲解节）");
    // 当年形态：1.18 起 #[repr(Rust)]（默认）结构体允许编译器重排字段
    // 以最小化填充；因此对这类结构体 transmute 更容易出问题（本就是 UB）。
    struct Mixed {
        small: u8,
        big: u64,
    }
    println!("Mixed 大小 = {} 字节（编译器可自由重排）", std::mem::size_of::<Mixed>());

    println!("\n6. windows_subsystem（println 讲解节）");
    // 当年形态：#![windows_subsystem = "windows"] 于 1.18 稳定（RFC 1665），
    // 用于 Windows GUI 程序隐藏控制台；crate 级属性，单文件演示不适用。
    println!("#![windows_subsystem]：控制 Windows 子系统链接选项。");
}

mod helper {
    // pub(crate)：对整个 crate 可见，但库用户不可见
    pub(crate) fn visible_in_crate() -> &'static str {
        "crate 内可见"
    }
}
