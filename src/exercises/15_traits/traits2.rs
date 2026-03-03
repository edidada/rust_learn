trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {  // ✅ 为 Vec<String> 实现 AppendBar
    fn append_bar(mut self) -> Self {  // ✅ 实现 append_bar 方法
        self.push(String::from("Bar"));  // ✅ 将字符串 "Bar" 推入向量
        self
    }
}

fn main() {
    // You can optionally experiment here.
    let vec = vec![String::from("Hello"), String::from("World")];
    let vec = vec.append_bar();
    println!("{:?}", vec);  // 输出: ["Hello", "World", "Bar"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
