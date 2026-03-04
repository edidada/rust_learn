// Type casting in Rust is done via the usage of the `as` operator.
// Note that the `as` operator is not only used when type casting. It also helps
// with renaming imports.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Make a conversion before dividing.
    // 将 usize 转换为 f64
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));

    // 更多测试用例 - 使用切片而不是固定大小的数组
    let test_cases = [
        (vec![1.0, 2.0, 3.0], 2.0),           // 平均值：2.0
        (vec![0.0, 0.0, 0.0, 0.0], 0.0),      // 平均值：0.0
        (vec![1.0], 1.0),                     // 单元素数组
        (vec![-1.0, 1.0], 0.0),               // 正负数
        (vec![10.0, 20.0, 30.0, 40.0], 25.0), // 平均值：25.0
    ];

    for (arr, expected) in test_cases {
        let result = average(&arr);
        println!("average({:?}) = {} (期望: {})", arr, result, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}