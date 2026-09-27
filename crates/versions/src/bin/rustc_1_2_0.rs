// rustc 1.2.0 演示 —— DST 强转 / iter::once / wrapping / Debug 构建器 / 毒锁
use std::fmt;

fn main() {
    println!("rustc 1.2.0 演示");

    println!("\n1. iter::once / iter::empty");
    let one: Vec<i32> = std::iter::once(42).collect();
    let empty: Vec<i32> = std::iter::empty().collect();
    println!("once -> {:?}, empty -> {:?}", one, empty);

    println!("\n2. 开启显式回绕：wrapping_div / wrapping_rem / wrapping_shl / wrapping_neg");
    // 默认算术不允许溢出；想回绕必须显式写。1.2 补齐了 div/rem/neg/shl/shr。
    let a = i32::MIN;
    let quo = a.wrapping_div(-1); // 溢出但按二进制补码回绕
    let rem = (-7).wrapping_rem(3);
    let shl = 1_u8.wrapping_shl(9); // shl 位数超宽，回绕到 1
    let neg = 0_u8.wrapping_neg();
    println!("i32::MIN/(-1) 回绕 = {}, -7%%3 = {}, 1<<9(u8) = {}, 0 翻转 = {}", quo, rem, shl, neg);

    println!("\n3. {:#?} 替代形式的 pretty-print Debug");
    let nested = vec![vec![1, 2], vec![3, 4]];
    println!("{:#?}", nested);

    println!("\n4. fmt::Formatter 的 debug 构建器");
    #[derive(Debug)]
    struct Person;
    // 用 debug_struct 自定义 Debug 输出
    struct Named {
        name: String,
        age: u8,
    }
    impl fmt::Debug for Named {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Named")
                .field("name", &self.name)
                .field("age", &self.age)
                .finish()
        }
    }
    let an = Named { name: "Lin".to_string(), age: 30 };
    println!("{:?}（Person 为空 Debug 提示） | 复杂结构 = {:?}", Person, an);

    println!("\n5. str::matches 子串迭代器（1.2 新增）");
    let ms: Vec<&str> = "ababab".matches("ab").collect();
    println!("'ababab'.matches(\"ab\") = {:?}", ms);
    println!("'Apple'.matches('p') = {:?}", "Apple".matches('p').collect::<Vec<&str>>());

    println!("\n6. Extend for String");
    let mut sb = String::new();
    sb.extend(['h', 'e', 'y']); // 1.2 起扩展支持 &char 等引用迭代器
    println!("extend(['h','e','y']) = {}", sb);

    println!("\n7. io::ErrorKind::InvalidData");
    let e = std::io::Error::new(std::io::ErrorKind::InvalidData, "my bad");
    println!("InvalidData? {} ({})", matches!(e.kind(), std::io::ErrorKind::InvalidData), e);

    println!("\n8. 处理毒锁：PoisonError 事件调研");
    // 1.2 起 PoisonError 提供 into_inner/get_ref/get_mut 取回锁操作员
    let lock = std::sync::Arc::new(std::sync::Mutex::new(0_u32));
    {
        let _l = lock.lock().unwrap();
        // 若这里 panic，锁将被"毒化"
    }
    let mut val = match lock.lock() {
        Ok(v) => v,
        // 若拿到毒化后的锁，仍可通过 into_inner 恢复数据
        Err(poisoned) => poisoned.into_inner(),
    };
    *val += 1;
    println!("毒锁事故对照成功，val = {}", *val);
}
