// rustc 1.30.0 演示 —— raw identifier + crate:: 路径 + 宏经 use 导入
// 该版本 introduces:
// 1) Language：raw identifier：r#for / r#fn 等，让关键字也能当标识符（预留 API 名时有用）。
// 2) Language：路径里可用 `crate::`：模块内 `use crate::foo;` 直指 crate 根；
//    外部 crate 用法不再需要 `::serde_json::...` 的 `::` 前缀。
// 3) 过程宏可用（派生/属性/函数式）—— 需 proc-macro crate，本例用 println 讲解。
// 4) 宏可用 `use` 从 crate 根导入（#[macro_export] 的宏进入 crate 根）。

// 模块：内部 use crate:: 指回 crate 根（1.30 前必须 use ::crate_name 或再显式路径）
mod helpers {
    // crate::greet 从 crate 根导入（1.30 前写法是 use ::greet;）
    use crate::greet;
    pub fn hello() {
        greet();
    }
}

// #[macro_export] 的宏位于 crate 根，可用 use 导入
#[macro_export]
macro_rules! shout {
    ($m:expr) => {
        println!("shout! -> {}", $m)
    };
}

fn greet() {
    println!("crate 根的 greet() 被 helpers 模块调用");
}

fn main() {
    println!("rustc 1.30.0 演示");

    // 1. raw identifier：关键字当标识符
    println!("\n1. raw identifier（r# 前缀）");
    let r#for = true;
    println!("let r#for = {}；这个变量名就是关键字 for", r#for);

    // 2. crate:: 路径
    println!("\n2. crate:: 路径");
    helpers::hello();

    // 3. 过程宏（讲解；真正的 proc-macro 要独立 crate）
    println!("\n3. 过程宏（println 讲解）");
    println!("1.30 起可写 proc-macro crate：#[derive(...)]、#[attr]、函数式宏（TokenStream 输入输出）");

    // 4. 宏经 use 导入（#[macro_export] 宏在 crate 根）
    println!("\n4. 宏的 use 导入");
    use crate::shout;
    shout!("macro 经 use 导入");

    // 5. trim_start / trim_end（对比旧名 trim_left/trim_right）
    println!("\n5. trim_start / trim_end");
    let s = "  rust  ";
    println!("trim_start = \"{}\"", s.trim_start()); // 旧名 trim_left
    println!("trim_end   = \"{}\"", s.trim_end());   // 旧名 trim_right
    println!("trim_start_matches(' ') = \"{}\"", s.trim_start_matches(' '));

    // 6. Iterator::find_map
    println!("\n6. Iterator::find_map");
    let found = ["12", "abc", "34"]
        .iter()
        .find_map(|x| x.parse::<i32>().ok());
    println!("find_map(可解析) = {:?}", found);

    // 7. Ipv4Addr 关联常量
    println!("\n7. Ipv4Addr::LOCALHOST 等关联常量");
    println!("LOCALHOST = {}", std::net::Ipv4Addr::LOCALHOST);
    println!("UNSPECIFIED = {}", std::net::Ipv4Addr::UNSPECIFIED);
}
