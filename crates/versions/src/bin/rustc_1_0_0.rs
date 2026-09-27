// rustc 1.0.0 演示 —— 1.0 基线：所有权 / 借用 / trait / 索引 / 拷贝
fn main() {
    println!("rustc 1.0.0 演示");

    println!("\n1. 所有权与移动（move 语义，1.0 即有）");
    // vec 拥有堆上数据；赋值给 moved 后所有权转移，原变量不可再用
    let vec = vec![10, 20, 30];
    let moved = vec; // 移动
    println!("moved = {}（原 vec 所有权已转移）", moved[0]);

    println!("\n2. 借用：不可变借用 vs 可变借用");
    let mut s = String::from("hello");
    let len = s.len(); // &self 只读借用
    s.push_str(" world"); // &mut self 可变借用
    println!("s = \"{}\"（len 读取借用在先，随后可变借用）", s);

    println!("\n3. 生命周期标注基线");
    let a = String::from("abc");
    let b = String::from("def");
    let longer = longest(&a, &b);
    println!("longest = {}", longer);

    println!("\n4. trait + 默认方法（1.0 起扩展 trait 已并入核心 trait）");
    // 1.0 把 IteratorExt 的方法并入 Iterator trait 本身，迭代器链从此开箱即用
    let sum: i32 = [1, 2, 3, 4].iter().filter(|x| **x % 2 == 0).sum();
    println!("偶数之和 = {}", sum);

    println!("\n5. UFCS：不带 trait 的关联路径 MyType::default()");
    struct Counter;
    impl Default for Counter {
        fn default() -> Self {
            Counter
        }
    }
    // 1.0 起可以直接写 Counter::default()
    let _c = Counter::default();
    println!("Counter::default() 通过 UFCS 调用成功");

    println!("\n6. Index/IndexMut 按值索引");
    let mut map = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    // 1.0 起 Index 以值方式取索引，"string" 字面量可直接索引
    println!("map[\"one\"] = {}", map["one"]);

    println!("\n7. Copy 继承 Clone");
    let n = 7;
    let m = n.clone(); // Copy 类型必然实现 Clone
    println!("n = {}, m = {}（Copy 类型 clone 后原值仍可用）", n, m);

    println!("\n8. match / Result / Option 基线");
    let r: Result<i32, String> = Ok(40);
    match r {
        Ok(v) => println!("match Ok({})", v),
        Err(_) => println!("Err 分支未进入"),
    }
    let o = Some("value");
    println!("option 匹配 = {}", o.unwrap_or("fallback"));

    println!("\n9. 当年老 API 与现行等价物的对照");
    // 当年形态：`std::thread::sleep_ms(1)`；毫秒计时 API 后来被 Duration 版取代
    // 现行等价物：`std::thread::sleep(std::time::Duration::from_millis(1))`
    std::thread::sleep(std::time::Duration::from_millis(1));
    println!("sleep 一毫秒：当年 thread::sleep_ms(1) -> 现行 thread::sleep(Duration)");
}

// 泛型生命周期：'a 表示两个参数中被持有较短者的寿命
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
