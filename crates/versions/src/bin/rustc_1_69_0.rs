// rustc 1.69.0 演示 —— CStr::from_bytes_until_nul、const SocketAddr、Rc 的 AsFd
fn main() {
    println!("rustc 1.69.0 演示");

    // ============================================================
    println!("\n1. CStr::from_bytes_until_nul");
    use std::ffi::{CStr, FromBytesUntilNulError};
    // 以 NUL 结尾的字节串 -> &CStr，无需再手写 unsafe
    let bytes: &[u8] = b"hello 1.69\0";
    let c = CStr::from_bytes_until_nul(bytes).unwrap();
    println!("  from_bytes_until_nul = {:?} -> {}", c, c.to_string_lossy());
    // 没有结尾 NUL 时返回 FromBytesUntilNulError
    let err: Result<&CStr, FromBytesUntilNulError> =
        CStr::from_bytes_until_nul(b"no_nul");
    println!("  无 NUL 结尾 -> Err: {}", err.err().unwrap());

    // ============================================================
    println!("\n2. const 上下文的 SocketAddr 系列");
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
    // 1.69 起 SocketAddr::new 等可在 const 中使用
    const SA: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    const V4: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 2), 22);
    const V6: SocketAddrV6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0);
    println!("  const SocketAddr::new = {}", SA);
    println!("  SA.is_ipv4() = {}, port = {}", SA.is_ipv4(), SA.port());
    println!("  const SocketAddrV4 = {}, ip = {}", V4, V4.ip());
    println!("  const SocketAddrV6 = {}, flowinfo/scope_id = {}/{}", V6, V6.flowinfo(), V6.scope_id());

    // ============================================================
    println!("\n3. Rc 实现 AsFd / AsRawFd（Unix；Windows 上对应 AsHandle/AsRawHandle）");
    use std::fs::File;
    use std::io::Read;
    use std::rc::Rc;
    // 1.69 起 Rc<File> 实现 AsFd/AsRawFd（Unix 上），可共享句柄包装；
    // Windows 上 std::os::fd 未启用，此处用现行等价物演示共享所有权。
    let f = File::open("Cargo.toml").expect("Cargo.toml 应存在（cwd=仓库根）");
    let rc = Rc::new(f);
    #[cfg(unix)]
    {
        use std::os::fd::{AsFd, AsRawFd};
        let fd = rc.as_fd();
        println!("  Rc<File>.as_fd() 的原始 fd = {:?}", fd.as_raw_fd());
    }
    #[cfg(not(unix))]
    println!("  （当前平台为非 Unix：AsFd 不可用，Windows 等价物为 std::os::windows::io::AsHandle）");
    let mut rc2 = rc.clone();
    let mut s = String::new();
    // &File 实现 Read（&self 方法），所以共享的 Rc<File> 也能只读读取
    let n = (&*rc2).read_to_string(&mut s).unwrap_or(0);
    println!("  通过 Rc<File> 读取了 {} 字节（Rc 共享同一底层句柄）", n);

    // ============================================================
    println!("\n4. packed 结构体上 derive 内建 trait（Copy 字段）");
    // 1.69 起若字段都是 Copy，packed 结构体可以直接 derive Clone/Copy/PartialEq 等
    #[repr(packed)]
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Packed {
        a: u8,
        b: u32,
    }
    let p1 = Packed { a: 1, b: 2 };
    let p2 = p1; // Copy
    println!("  Packed {:?} == 副本: {}", p1, p1 == p2);
    // 注意：读取 packed 字段仍需按对齐规则拷贝（`{ p1.a }` 形式），不能直接取引用。

    // ============================================================
    println!("\n5. 兼容性提醒");
    println!("  - packed 字段的非对齐引用是硬错误（&packed.field 不再容忍）");
    println!("  - repr(C) 枚举默认 c_int 大小");
    println!("  - Cargo: 建议使用 cargo fix / cargo clippy --fix / cargo add");
}
