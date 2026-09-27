// rustc 1.34.2 演示 —— 安全补丁：收回 Error::type_id（CVE-2019-12083）
// 该版本 introduces:
// 1.34.0 曾把 Error::type_id 稳定化，但它可被滥用于构造内存不安全调用（CVE-2019-12083）。
// 1.34.2 把 Error::type_id 重新标记为 unstable（收回稳定）。
// 现代版本中判断错误具体类型应使用 downcast_ref 等安全 API。

fn main() {
    println!("rustc 1.34.2 演示");
    println!("\n1. 安全修复（讲解）");
    println!("CVE-2019-12083：Error::type_id 被收回稳定化（1.34.0 误稳定）");
    println!("\n2. 现行安全等价物：downcast_ref 判断错误类型");
    use std::error::Error;
    use std::fmt;
    #[derive(Debug)]
    struct MyErr;
    impl fmt::Display for MyErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyErr")
        }
    }
    impl Error for MyErr {}
    let e: Box<dyn Error> = Box::new(MyErr);
    println!("e.downcast_ref::<MyErr>().is_some() = {}", e.downcast_ref::<MyErr>().is_some());
}
