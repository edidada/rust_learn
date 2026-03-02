// break 关键字示例

fn main() {
    // 1. 基本的break用法
    println!("1. Basic break:");
    for i in 0..10 {
        if i == 5 {
            break; // 当i等于5时退出循环
        }
        print!("{} ", i);
    }
    println!();
    
    // 2. 使用标记的break
    println!("2. Labeled break:");
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i + j == 3 {
                break 'outer; // 退出外部循环
            }
            print!("({},{}) ", i, j);
        }
        println!();
    }
    
    // 3. 在loop中使用break
    println!("3. Break in loop:");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break; // 退出无限循环
        }
        print!("{} ", counter);
    }
    println!();
}