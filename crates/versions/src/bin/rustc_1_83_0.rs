// rustc 1.83.0 演示 —— const 内 &mut/可变指针、Entry::insert_entry、ControlFlow、skip_until、ErrorKind
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.83 稳定的 API 在 1.98 均可用。
use std::io::BufRead;
use std::ops::ControlFlow;

// 1.83 重点：const fn 中可以使用 &mut / *mut。
// 旧行为：const fn 里拿可变引用直接报错（"mutable references in const" 需要 nightly）；
// 新行为：编译期求值域内的"局部可变"完全合法（const 的值仍在编译期确定）。
const fn bump_to(mut n: u32) -> u32 {
    let r = &mut n; // ← 1.83 起 const 中可拿 &mut
    *r += 10;
    n
}
const BUMPED: u32 = bump_to(5); // 编译期执行 &mut 逻辑

// const 中创建引用 + 修改（Cell 在 const 中也可用）：
const SQUARED: i32 = {
    let mut x = 6;
    let m = &mut x; // const 内的局部 &mut
    *m = *m * *m;
    x
};

fn main() {
    println!("rustc 1.83.0 演示");

    println!("\n1. const 内 &mut / 可变指针（1.83 重点）");
    println!("const BUMPED = {BUMPED}（const fn 里 &mut += 10）");
    println!("const SQUARED = {SQUARED}（const 块里 &mut 平方）");

    println!("\n2. hash_map::Entry::insert_entry（1.83 稳定）");
    let mut map = std::collections::HashMap::from([("a", 1)]);
    // 旧写法：match entry { Vacant(e) => e.insert(v), Occupied(..) => .. }
    // insert_entry 一步插入/覆盖并返回 &mut 新值，可直接就地修改：
    let mut v = map.entry("b").insert_entry(0); // 返回 OccupiedEntry，借到 map 上
    *v.get_mut() += 5; // 就地修改返回的 entry
    println!("map = {map:?}（\"b\" 插入后经 OccupiedEntry::get_mut 改成 5）");

    println!("\n3. ControlFlow::break_value / map_break / map_continue");
    let cf: ControlFlow<i32, u8> = ControlFlow::Break(7);
    // 旧写法：match cf { ControlFlow::Break(b) => Some(b), Continue(_) => None }
    println!("break_value() = {:?}", cf.break_value());
    let cf2: ControlFlow<u8, u8> = ControlFlow::Continue(3);
    println!("continue_value() = {:?}", cf2.continue_value());
    let mapped = cf2.map_continue(|v| v * 10).map_break(|b| b as u8 + 1);
    println!("map_continue(×10) = {mapped:?}");

    println!("\n4. BufRead::skip_until（1.83 稳定）");
    let data = std::io::Cursor::new(b"skip-me:rest-of-line\nnext");
    let mut data = std::io::BufReader::new(data);
    data.skip_until(b':').unwrap(); // 跳过到 ':' 为止
    let mut out = String::new();
    data.read_line(&mut out).unwrap();
    println!("skip_until(b':') 后读到 = {:?}", out.trim_end());

    println!("\n5. ErrorKind 新变体（1.83 稳定）");
    // 这些错误种类此前只是 libc errno，没有稳定映射；现在可精确匹配：
    let e = std::io::Error::from_raw_os_error(122 /* Windows ERROR_DISK_FULL */);
    println!("io::Error(122) kind = {:?}", e.kind());

    println!("\n6. const 上下文新能力讲解（Option::unwrap、str::as_bytes_mut…）");
    // 1.83 起 Option::unwrap、mem::replace、slice::split_at_mut 等一大批 API 可用于 const。
    const OPT: Option<u32> = Some(11);
    const VAL: u32 = OPT.unwrap(); // ← const 中 unwrap，1.83 之前不行
    println!("const VAL = {VAL}（const 中 Option::unwrap）");
    // str::as_bytes_mut 等"可变性"系列同样进入 const（讲解性，运行期演示）：
    let mut s = String::from("hello");
    let bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    bytes[0] = b'H';
    println!("as_bytes_mut 改后 = {s}");
}
