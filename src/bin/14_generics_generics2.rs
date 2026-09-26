// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {  // ✅ 添加泛型参数
    value: T,        // ✅ 使用泛型类型
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {  // ✅ 为泛型实现
    fn new(value: T) -> Self {  // ✅ 参数类型改为泛型
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    // 测试包装不同类型的值
    let int_wrapper: Wrapper<u32> = Wrapper::new(42);
    println!("整数包装器: {}", int_wrapper.value);

    let str_wrapper: Wrapper<&str> = Wrapper::new("Foo");
    println!("字符串包装器: {}", str_wrapper.value);

    let float_wrapper: Wrapper<f64> = Wrapper::new(3.14);
    println!("浮点数包装器: {}", float_wrapper.value);

    let vec_wrapper: Wrapper<Vec<i32>> = Wrapper::new(vec![1, 2, 3]);
    println!("向量包装器: {:?}", vec_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
