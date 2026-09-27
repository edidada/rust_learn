// rustc 1.1.0 演示 —— fs 扩充 / split_whitespace / mpsc 迭代 / 切片特化
fn main() {
    println!("rustc 1.1.0 演示");

    println!("\n1. str::split_whitespace（1.0 曾用 words，1.1 明确按 Unicode 空白切分）");
    let words: Vec<&str> = "  a   b\u{3000}c ".split_whitespace().collect();
    println!("切分结果 = {:?}", words);

    println!("\n2. mpsc::Receiver 转 IntoIterator");
    // 当年形态：mpsc::Receiver 的 into_iter 已在 1.1 稳定，如今依旧可用
    let (tx, rx) = std::sync::mpsc::channel();
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    drop(tx);
    let collected: Vec<i32> = rx.into_iter().collect();
    println!("迭代 channel = {:?}", collected);

    println!("\n3. From<u32> 构造 Ipv4Addr");
    let addr = std::net::Ipv4Addr::from(0x7f000001_u32);
    println!("0x7f000001 -> {}", addr);

    println!("\n4. 切片 count/nth/last 覆盖为 O(1)");
    let v = [1, 2, 3, 4, 5];
    // 对切片而言这些方法不再逐个迭代，直接 O(1) 计算
    println!("v.iter().count() = {}, nth(2) = {:?}, last = {:?}",
        v.iter().count(), v.iter().nth(2), v.iter().last());

    println!("\n5. 原始句柄转换 trait 说明");
    // 当年形态（1.1 稳定）：File/TcpStream 等实现 AsRawFd/FromRawFd（Unix）与
    // AsRawHandle/FromRawHandle（Windows），把 std 类型转为底层系统句柄。
    // 句柄转换依赖平台目标，此处仅讲解；用一次完整 I/O 往返示意句柄类 IO 的可用性：
    let mut buf: Vec<u8> = Vec::new();
    use std::io::Write;
    write!(buf, "fd-handle-io").unwrap();
    println!("IO 类型仍可底层转换：{}", String::from_utf8_lossy(&buf));
}
