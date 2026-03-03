// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // 将第一个字符大写，然后加上剩余字符
            first.to_uppercase().collect::<String>() + chars.as_str()
        }
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 使用 map 将 capitalize_first 应用到每个元素
    words.iter()
        .map(|&word| capitalize_first(word))
        .collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // 使用 map 和 collect 连接字符串
    words.iter()
        .map(|&word| capitalize_first(word))
        .collect()
}

fn main() {
    // You can optionally experiment here.
    println!("{}", capitalize_first("hello"));  // Hello
    println!("{}", capitalize_first(""));       // 空字符串

    let words = vec!["hello", "world"];
    println!("{:?}", capitalize_words_vector(&words));  // ["Hello", "World"]

    let words2 = vec!["hello", " ", "world"];
    println!("{}", capitalize_words_string(&words2));  // Hello World
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
