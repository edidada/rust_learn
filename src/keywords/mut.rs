// mut 关键字示例

fn main() {
    // 1. 可变变量
    let mut x = 5;
    println!("Initial x: {}", x);
    x = 10;
    println!("Updated x: {}", x);
    
    // 2. 可变引用
    let mut y = 20;
    let y_ref = &mut y;
    *y_ref = 30;
    println!("y after mutable reference: {}", y);
    
    // 3. 可变参数
    fn increment(mut value: i32) -> i32 {
        value += 1;
        value
    }
    
    let z = 40;
    let result = increment(z);
    println!("Original z: {}, Incremented: {}", z, result);
    
    // 4. 可变向量
    let mut vec = vec![1, 2, 3];
    println!("Initial vec: {:?}", vec);
    vec.push(4);
    vec[0] = 10;
    println!("Updated vec: {:?}", vec);
}