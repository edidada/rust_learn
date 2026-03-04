// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // 接收元组向量，返回字符串向量
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(count) => {
                    let mut result = string;
                    for _ in 0..count {
                        result.push_str("bar");
                    }
                    result
                }
            };
            output.push(result);
        }

        output
    }
}

// 为了方便测试，重新导出函数
pub use my_module::transformer;

fn main() {
    // You can optionally experiment here.
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];

    let output = transformer(input);

    println!("转换结果:");
    for (i, s) in output.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // 导入主模块中的 transformer 函数
    use super::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }

    #[test]
    fn test_uppercase() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            ("WORLD".to_string(), Command::Uppercase),
            ("MixedCase".to_string(), Command::Uppercase),
        ];
        let output = transformer(input);

        assert_eq!(output, ["HELLO", "WORLD", "MIXEDCASE"]);
    }

    #[test]
    fn test_trim() {
        let input = vec![
            ("  hello  ".to_string(), Command::Trim),
            ("\tworld\t".to_string(), Command::Trim),
            ("  multiple  spaces  ".to_string(), Command::Trim),
        ];
        let output = transformer(input);

        assert_eq!(output, ["hello", "world", "multiple  spaces"]);
    }

    #[test]
    fn test_append() {
        let input = vec![
            ("foo".to_string(), Command::Append(0)),
            ("bar".to_string(), Command::Append(1)),
            ("test".to_string(), Command::Append(3)),
        ];
        let output = transformer(input);

        assert_eq!(output, ["foo", "barbar", "testbarbarbar"]);
    }
}