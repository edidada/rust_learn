// const 关键字示例

// 1. 编译时常量
const MAX_VALUE: i32 = 100;
const PI: f64 = 3.141592653589793;

// 2. 编译时可计算函数
const fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 3. 编译时块
const COMPUTED: i32 = {
    let x = 10;
    let y = 20;
    x + y
};

fn main() {
    println!("MAX_VALUE: {}", MAX_VALUE);
    println!("PI: {}", PI);
    println!("COMPUTED: {}", COMPUTED);
    
    // 使用const函数
    const SUM: i32 = add(5, 7);
    println!("SUM: {}", SUM);
    
    // 运行时也可以使用
    let runtime_sum = add(10, 20);
    println!("Runtime sum: {}", runtime_sum);
}