// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String>  {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())  // ✅ 返回 Err
    } else {
        Ok(format!("Hi! My name is {name}"))  // ✅ 返回 Ok
    }
}

fn main() {
    // You can optionally experiment here.
    let test_names = vec!["Alice", "", "Bob"];

    for name in test_names {
        match generate_nametag_text(name.to_string()) {
            Ok(text) => println!("成功: {}", text),
            Err(err) => println!("失败: {} -> 错误: {}", name, err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
