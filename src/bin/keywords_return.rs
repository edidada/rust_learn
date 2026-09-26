// return 关键字示例

// 1. 基本的return用法
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// 2. 提前返回
fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        return None; // 提前返回
    }
    Some(a as f64 / b as f64)
}

// 3. 在循环中返回
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number); // 找到后立即返回
        }
    }
    None
}

// 4. 无返回值的return
fn print_positive_number(number: i32) {
    if number <= 0 {
        return; // 提前返回，无返回值
    }
    println!("Positive number: {}", number);
}

fn main() {
    // 1. 基本返回
    let sum = add(5, 7);
    println!("5 + 7 = {}", sum);
    
    // 2. 提前返回
    match divide(10, 2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    match divide(10, 0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    // 3. 在循环中返回
    let numbers = [1, 3, 5, 7, 8, 9];
    match find_first_even(&numbers) {
        Some(even) => println!("First even number: {}", even),
        None => println!("No even numbers found"),
    }
    
    // 4. 无返回值的return
    print_positive_number(42);
    print_positive_number(-10); // 不会打印任何内容
}