// rustc 1.39.0 演示 —— async fn / .await / async move 块（本版唯一允许使用 tokio 的版本）
// 1.39.0 稳定了 async/await：async fn 返回的是一个实现 Future 的匿名类型，
// 只有在运行时（这里用 tokio）里被 .await 或驱动时才执行。
use std::time::Instant;

// async fn 返回 Future，await 处"暂停"而不是阻塞线程。
async fn say_after(msg: &'static str, ms: u64) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    format!("{} (after {}ms)", msg, ms)
}

#[tokio::main] // 1.39 语言层稳定后，运行时仍由外部 crate（tokio）提供
async fn main() {
    println!("rustc 1.39.0 演示（async/await）");

    println!("\n1. async fn / .await 基础");
    let start = tokio::time::Instant::now();
    let r = say_after("hello from async fn", 50).await; // .await 会在此挂起，期间线程不被阻塞
    println!("await 结果: {}", r);
    println!("耗时约 {} ms，说明 sleep 真的是异步等待", start.elapsed().as_millis());

    println!("\n2. tokio::time::sleep 并发等待");
    let t0 = std::time::Instant::now();
    // 两个 future 用 tokio::join! 同时驱动，总耗时约等于最长的那个（60ms），而不是相加（110ms）
    let (a, b) = tokio::join!(
        say_after("任务A", 60),
        say_after("任务B", 50),
    );
    println!("并发结果: A = \"{}\", B = \"{}\"", a, b);
    println!("join! 总耗时 ≈ {}ms（若串行将是 ~110ms，这就是 .await 调度的意义）", t0.elapsed().as_millis());

    println!("\n3. async move 块 / async 表达式");
    // async move {} 把环境 move 进闭包式 Future；async {} 则是不带捕获的延迟求值块。
    let owned = String::from("被 move 进 future 的数据");
    let mut fut = std::pin::pin!(async move {
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        format!("move 块返回: {} 长度={}", owned, owned.len())
    });
    let out = fut.as_mut().await;
    println!("{}", out);
    // async {} 不捕获环境，等价于"延迟执行的代码块"
    let expr = async { std::mem::size_of::<u64>() * 2 };
    println!("async {{ }} 表达式的值 = {}", expr.await);

    println!("\n4. match 卫语句中对 move 绑定取共享引用（1.39 允许）");
    let arr: Box<[u8; 4]> = Box::new([1, 2, 3, 4]);
    match arr {
        // nums 被 move 绑定；1.39 起允许在卫语句中 nums.iter()（隐式共享借用）
        nums if nums.iter().sum::<u8>() == 10 => {
            println!("sum = 10 成立，box 数组仍归本分支所有: {:?}", nums);
            drop(nums); // 拥有权还在，drop 合法
        }
        _ => unreachable!(),
    }
    println!("不存在借用冲突：卫语句借用已随分支结束而结束");
}
