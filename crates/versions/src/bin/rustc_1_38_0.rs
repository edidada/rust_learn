// rustc 1.38.0 演示 —— 欧几里得除法/取余 / type_name / Duration 浮点 API / join &[T] / ptr::cast
fn main() {
    println!("rustc 1.38.0 演示");
    // 1.38.0 引入：欧几里得除法/取余。与 `/`、`%` 的"截断"语义不同，
    // Euclidean 语义保证余数非负：div_euclid 向下取整到满足 rem >= 0，rem_euclid 总返回非负。
    println!("\n1. div_euclid / rem_euclid");
    println!("-7 / 2   = {} , -7 % 2      = {}（截断语义，余数可为负）", -7 / 2, -7 % 2);
    println!("-7.div_euclid(2)   = {}（floor 语义，-4）", (-7i32).div_euclid(2));
    println!("-7.rem_euclid(2)   = {}（非负余数，1）", (-7i32).rem_euclid(2));
    println!("7.rem_euclid(-2)   = {}（对负除数也保证数学意义上的余数）", 7i32.rem_euclid(-2));

    // 1.38.0 稳定：any::type_name —— 拿到类型的人类可读名字（仅调试用，不能保证稳定格式）。
    println!("\n2. any::type_name");
    fn type_of<T: ?Sized>(_t: &T) -> &'static str {
        std::any::type_name::<T>()
    }
    println!("type_of(&\"hello\") = {}", type_of("hello"));
    println!("type_of(HashMap)  = {}", std::any::type_name::<std::collections::HashMap<String, i32>>());

    // 1.38.0 稳定：Duration 系列浮点 API（as_secs_f32/f64、from_secs_f32/f64、mul/div_f32/f64）。
    println!("\n3. Duration 浮点 API");
    use std::time::Duration;
    let d = Duration::from_millis(1500);
    println!("1.5s 的 as_secs_f64 = {}", d.as_secs_f64());
    let half = d.mul_f64(0.5); // 1.38 稳定：Duration * f64
    println!("d.mul_f64(0.5) = {:?}", half);
    let restored = Duration::from_secs_f64(0.75); // 1.38 稳定：秒(f64) → Duration
    println!("from_secs_f64(0.75) = {:?}", restored);

    // 1.38.0 变更：slice::join（及 concat/connect）现在除 &T 外也接受 &[T] 切片元素。
    println!("\n4. slice::{join} 接受 &[T]");
    let pieces: &[&[&str]] = &[&["a", "b"], &["c"]];
    let flat: Vec<&str> = pieces.concat(); // concat 本身在 1.38 支持 &[T] 元素
    println!("slice concat → {:?}", flat);
    let joined: String = pieces.concat().join("-");
    println!(" joined(\"-\")            = \"{}\"", joined);

    // 1.38.0 稳定：<*const T>::cast —— 隐式转换到任意 usize 大小的裸指针类型（替代强转 as *const X）。
    println!("\n5. <*const T>::cast");
    let bytes: [u8; 4] = [1, 2, 3, 4];
    let p: *const u8 = bytes.as_ptr();
    let p32: *const u32 = p.cast::<u32>(); // 1.38 稳定前只能写 `as *const u32`
    unsafe {
        println!("p.cast::<u32>() 读取 = {}", *p32); // 小端机器上即 0x04030201
    }
}
