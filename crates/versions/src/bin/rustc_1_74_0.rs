// rustc 1.74.0 演示 —— io::Error::other、Saturating、数组->Vec/Rc/Arc、const transmute_copy
fn main() {
    println!("rustc 1.74.0 演示");

    // ============================================================
    println!("\n1. std::io::Error::other 与 core::num::Saturating");
    // Error::other：任意错误包成 io::Error（以前只能用 io::Error::new(ErrorKind::Other, e)）
    let e: std::io::Error = std::io::Error::other("自定义错误");
    println!("  Error::other -> {}（kind = {:?}）", e, e.kind());
    // Saturating：包装整数，所有运算饱和
    use std::num::Saturating;
    let a = Saturating(u32::MAX);
    let b = a + Saturating(1u32);
    println!("  Saturating(MAX) + 1 = {}", b.0); // 仍是 MAX，不回绕
    println!("  Saturating(0) - 1 = {}", (Saturating(0u32) - Saturating(1u32)).0); // 0

    // ============================================================
    println!("\n2. From<[T; N]> for Vec/Rc<[T]>/Arc<[T]> 与 From<&[T;N]> for Vec");
    let v: Vec<i32> = (&[1, 2, 3]).into(); // 1.74 新 impl
    println!("  &array -> Vec = {:?}", v);
    let rc: std::rc::Rc<[i32]> = [1, 2, 3].into();
    let arc: std::sync::Arc<[i32]> = [4, 5].into();
    println!("  array -> Rc<[T]> = {:?}, Arc<[T]> = {:?}", rc, arc);

    // ============================================================
    println!("\n3. impl TryFrom<char> for u16");
    // BMP（<= 0xFFFF）内码点可转 u16；否则 Err
    let a: u16 = 'A'.try_into().unwrap();
    println!("  'A' -> u16 = {} (0x{:04X})", a, a);
    let emoji: Result<u16, _> = '🦀'.try_into(); // U+1F980 超出 BMP
    println!("  '🦀' -> u16 = Err: {}", emoji.is_err());

    // ============================================================
    println!("\n4. const 上下文 transmute_copy / is_ascii");
    // transmute_copy 在 const 中稳定（无借用，按大小拷贝；目标大小须 <= 源大小）
    const fn bytes_as_u16(x: [u8; 2]) -> u16 {
        unsafe { std::mem::transmute_copy(&x) }
    }
    const T: u16 = bytes_as_u16([0x34, 0x12]);
    println!("  const transmute_copy([u8;2] -> u16) = 0x{:04X}", T);
    const S: bool = "const str".is_ascii(); // 1.74 const 稳定
    const B: bool = [1u8, 2, 3].is_ascii();
    println!("  const str::is_ascii = {}, const [u8]::is_ascii = {}", S, B);

    // ============================================================
    println!("\n5. OsStr::as_encoded_bytes / OsString::into_encoded_bytes");
    use std::ffi::{OsStr, OsString};
    let os = OsStr::new("编码字节");
    println!("  OsStr::as_encoded_bytes = {:?}", os.as_encoded_bytes());
    let oss = OsString::from("转回字节");
    println!("  OsString::into_encoded_bytes = {:?}", oss.into_encoded_bytes());
    let round: OsString = unsafe { OsString::from_encoded_bytes_unchecked(os.as_encoded_bytes().to_vec()) };
    println!("  from_encoded_bytes_unchecked 回转 = {:?}", round);

    // ============================================================
    println!("\n6. impl From<io::Stdout> for Stdio");
    use std::process::{Command, Stdio};
    // 子进程的 stdout 继承当前进程的 stdout
    let child = Command::new("cmd")
        .args(["/C", "echo 使用父进程 stdout"])
        .stdout(Stdio::from(std::io::stdout()))
        .output();
    println!("  Stdio::from(stdout) 构造成功: {:?}", child.is_ok());

    // ============================================================
    println!("\n7. 文字说明（Cargo/兼容性）");
    println!("  - [lints] 表稳定：Cargo.toml 中统一配置 lint 级别");
    println!("  - --keep-going 稳定；cargo clean --dry-run；cargo update --recursive");
    println!("  - Cell::swap 部分重叠时 panic；--extern 拒绝非法 crate 名");
    println!("  - 允许显式 #[repr(Rust)]；private_in_public 拆分为 private_interfaces/bounds");
}
