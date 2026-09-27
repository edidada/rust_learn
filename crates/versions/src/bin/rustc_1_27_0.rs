// rustc 1.27.0 演示 —— dyn Trait 语法 + must_use + try_fold
// 该版本 introduces:
// 1) Language：`dyn Trait` 语法稳定：&Trait == &dyn Trait、Box<Trait> == Box<dyn Trait>，
//    与 `impl Trait` 形成可读对照（dyn=运行时多态，impl=静态分发）。
// 2) `#[must_use]` 用于函数；`proc` 不再保留。
// 3) x86 SIMD intrinsics + is_x86_feature_detected! 稳定（本例 cfg 限定 x86_64）。
// 4) 一批 Stabilized APIs：try_fold/try_for_each、Option::filter、HashMap::remove_entry 等。

#[must_use]
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("rustc 1.27.0 演示");

    // 1. dyn Trait：动态分发的显式写法（此前裸写 Box<Speaker>）
    println!("\n1. dyn Trait 语法");
    trait Speaker {
        fn speak(&self) -> &'static str;
    }
    impl Speaker for &'static str {
        fn speak(&self) -> &'static str {
            self
        }
    }
    let s: Box<dyn Speaker> = Box::new("dyn Trait 上场");
    println!("Box<dyn Speaker> -> {}", s.speak());

    // 2. #[must_use] fn：必须消费返回值（此处正常使用避免警告）
    println!("\n2. #[must_use] 函数");
    println!("add_one(9) = {}", add_one(9)); // 若写 add_one(9); 会得到 warning

    // 3. Iterator::try_fold / try_for_each：短路式迭代
    println!("\n3. try_fold / try_for_each");
    let sum = [1, 2, 3, 4]
        .iter()
        .try_fold(0i64, |acc, &x| {
            if acc + x > 100 {
                Err(acc)
            } else {
                Ok(acc + x)
            }
        });
    println!("try_fold 结果 = {:?}", sum);
    let stop = [1, 2, 0, 4].iter().try_for_each(|&x| if x == 0 { Err(()) } else { Ok(()) });
    println!("try_for_each 遇 0 短路: {:?}", stop);

    // 4. Option::filter / HashMap::remove_entry
    println!("\n4. Option::filter / remove_entry");
    let even = Some(4).filter(|x| x % 2 == 0);
    println!("Some(4).filter(偶) = {:?}", even);
    let mut m: std::collections::HashMap<&str, i32> =
        [("a", 1), ("b", 2)].into_iter().collect();
    let removed = m.remove_entry("a");
    println!("remove_entry(\"a\") = {:?}", removed);
    println!("删除后 map = {:?}", m);

    // 5. Duration 精度 API
    println!("\n5. Duration::from_micros / subsec_millis");
    let d = std::time::Duration::from_micros(1_500_500);
    println!("1_500_500µs = {}.{:03}ms；subsec_millis = {}",
        d.as_secs(), d.subsec_millis(), d.subsec_millis());

    // 6. x86 SIMD 检测宏（1.27 稳定；cfg 限定到 x86/x86_64）
    println!("\n6. is_x86_feature_detected!");
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            println!("本机支持 sse2");
        }
        if is_x86_feature_detected!("avx2") {
            println!("本机支持 avx2");
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        println!("非 x86_64 目标，跳过检测");
    }

    // 7. 原子类型 Debug 输出行为变化：只打印内部值
    println!("\n7. 原子类型 Debug 变化");
    println!("{:?}", std::sync::atomic::AtomicBool::new(true));
    println!("{:?}", std::sync::atomic::AtomicUsize::new(5));
}
