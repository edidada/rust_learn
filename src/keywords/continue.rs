// continue 关键字示例

fn main() {
    // 1. 基本的continue用法
    println!("1. Basic continue:");
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        print!("{} ", i);
    }
    println!();
    
    // 2. 使用标记的continue
    println!("2. Labeled continue:");
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                continue 'outer; // 跳过外部循环的当前迭代
            }
            print!("({},{}) ", i, j);
        }
        println!();
    }
    
    // 3. 在loop中使用continue
    println!("3. Continue in loop:");
    let mut counter = 0;
    let mut sum = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        }
        if counter % 3 == 0 {
            continue; // 跳过能被3整除的数
        }
        sum += counter;
        print!("{} ", counter);
    }
    println!();
    println!("Sum of numbers not divisible by 3: {}", sum);
}