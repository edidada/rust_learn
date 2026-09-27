#[derive(Debug, PartialEq, Eq, Clone)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {

    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    // 检查整数溢出：i64::MIN / -1 会溢出
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    // 检查是否能整除
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.iter().copied().map(|n| divide(n, 27));

    // 收集结果，如果所有都成功则返回数组
    let results: Vec<Result<i64, DivisionError>> = division_results.collect();

    // 检查是否有错误
    for result in &results {
        if result.is_err() {
            // 如果出错，返回具体的错误
            return match result {
                Err(DivisionError::DivideByZero) => Err(DivisionError::DivideByZero),
                Err(DivisionError::IntegerOverflow) => Err(DivisionError::IntegerOverflow),
                Err(DivisionError::NotDivisible) => Err(DivisionError::NotDivisible),
                _ => unreachable!(),
            };
        }
    }

    // 将 Vec<Result<i64, DivisionError>> 转换为 [i64; 4]
    let array: [i64; 4] = [
        results[0].as_ref().unwrap().clone(),
        results[1].as_ref().unwrap().clone(),
        results[2].as_ref().unwrap().clone(),
        results[3].as_ref().unwrap().clone(),
    ];

    Ok(array)
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> [Result<i64, DivisionError>; 4] {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.iter().copied().map(|n| divide(n, 27));

    // 收集到数组中
    let results: Vec<Result<i64, DivisionError>> = division_results.collect();

    // 转换为固定大小的数组
    [
        results[0].clone(),
        results[1].clone(),
        results[2].clone(),
        results[3].clone(),
    ]
}

fn main() {
    // You can optionally experiment here.
    println!("测试 divide 函数:");
    println!("81 / 9 = {:?}", divide(81, 9));
    println!("81 / 0 = {:?}", divide(81, 0));
    println!("i64::MIN / -1 = {:?}", divide(i64::MIN, -1));
    println!("81 / 6 = {:?}", divide(81, 6));

    println!("\n测试 result_with_list:");
    println!("{:?}", result_with_list());

    println!("\n测试 list_of_results:");
    println!("{:?}", list_of_results());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
