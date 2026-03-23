// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()  // ✅ 将 &str 转换为 String
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
