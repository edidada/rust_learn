// rustc 1.72.0 演示 —— String::leak、TryFrom<&OsStr>、const CStr、Ipv6Addr Display
fn main() {
    println!("rustc 1.72.0 演示");

    // ============================================================
    println!("\n1. String::leak");
    // 1.72 稳定：把 String 泄漏成 &'static mut str（无 unsafe）
    let s = String::from("我泄漏了");
    let leaked: &'static mut str = s.leak();
    println!("  leak 后字符串 = {}", leaked);
    leaked.make_ascii_uppercase(); // &'static mut str 还可以修改
    println!("  改大写后 = {}", leaked);

    // ============================================================
    println!("\n2. impl TryFrom<&OsStr> for &str");
    use std::ffi::OsStr;
    let os: &OsStr = OsStr::new("utf-8 路径");
    let ok: Result<&str, _> = os.try_into();
    println!("  合法 UTF-8 -> {:?}", ok);
    // 非法 UTF-8 场景：构造含非法编码的 OsStr 需要 OsStr::from_bytes（Unix 专属）或
    // 1.74 的 from_encoded_bytes_unchecked，此处从简，仅文字说明转换会返回 Err
    println!("  含非法编码的 OsStr -> try_into 返回 Err（错误信息含无效字节位置）");

    // ============================================================
    println!("\n3. const 上下文的 CStr 系列 API");
    // 1.72 起 CStr 的 from_bytes_with_nul/to_bytes/to_str 等可在 const 中使用
    const C: &[u8] = b"const cstr\0";
    const CS: &std::ffi::CStr = match std::ffi::CStr::from_bytes_with_nul(C) {
        Ok(c) => c,
        Err(_) => panic!("bad"),
    };
    const TEXT: &str = match CS.to_str() {
        Ok(t) => t,
        Err(_) => panic!("bad"),
    };
    println!("  const to_str = {:?}", TEXT);
    println!("  const to_bytes = {:?}", CS.to_bytes());

    // ============================================================
    println!("\n4. Ipv6Addr Display 变更（IPv4 兼容地址）");
    use std::net::Ipv6Addr;
    // IPv4 兼容地址（低 32 位为 IPv4，前缀全零）以前显示为 ::127.0.0.1，
    // 1.72 起改为标准 IPv6 十六进制显示
    let a: Ipv6Addr = "::127.0.0.1".parse().unwrap();
    println!("  ::127.0.0.1 -> Display = {}", a);
    let mapped: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
    println!("  ::ffff:127.0.0.1 -> Display = {}（IPv4 映射仍用点分）", mapped);

    // ============================================================
    println!("\n5. Rc::ptr_eq 忽略指针元数据");
    let v1: std::rc::Rc<String> = std::rc::Rc::new("相同值".to_owned());
    let v2 = std::rc::Rc::clone(&v1);
    let s1 = std::rc::Rc::new("相同值".to_owned());
    println!("  同源 clone -> ptr_eq = {}", std::rc::Rc::ptr_eq(&v1, &v2));
    println!("  不同对象但值相同 -> ptr_eq = {}（比较的是数据地址而非值）",
        std::rc::Rc::ptr_eq(&v1, &s1));
    // 1.72 起：Rc<[T]> / Rc<str> 的切片元数据不再参与比较

    // ============================================================
    println!("\n6. 行为说明（文字）");
    println!("  - 上提 lint：invalid_nan_comparisons、invalid_reference_casting、");
    println!("    invalid_from_utf8(_unchecked)、undropped_manually_drops");
    println!("  - const 求值上限变为 lint + 指数退避警告，不再直接 hard error");
    println!("  - Arc/Rc/Weak::ptr_eq 忽略指针元数据；TypeId 哈希改为 128 位");
}
