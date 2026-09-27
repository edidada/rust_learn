fn main() {
    let my_option: Option<&str> = None;

    // 修复1: 使用 if let Some(value) 而不是 is_none() + unwrap()
    if let Some(value) = my_option {
        println!("{}", value);
    }

    // 修复2: 在 -3 后面添加逗号
    let my_arr = &[
        -1, -2, -3,  // 这里添加了逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // 修复3: 使用 clear() 而不是 resize(0, 5) 来清空向量
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();  // 使用 clear() 替代 resize(0, 5)
    println!("This Vec is empty, see? {my_vec:?}");

    // 修复4: 使用 std::mem::swap 来交换两个值
    let mut value_a = 45;
    let mut value_b = 66;
    // 正确交换两个值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}