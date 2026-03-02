// union 关键字示例

// 定义一个C风格的联合
union MyUnion {
    i: i32,
    f: f32,
    b: bool,
}

fn main() {
    println!("Using union keyword example");
    
    // 创建联合实例
    let mut u = MyUnion { i: 42 };
    
    // 读取整数字段
    unsafe {
        println!("Integer value: {}", u.i);
    }
    
    // 写入浮点数字段
    unsafe {
        u.f = 3.14;
        println!("Float value: {}", u.f);
    }
    
    // 写入布尔字段
    unsafe {
        u.b = true;
        println!("Boolean value: {}", u.b);
    }
    
    // 注意：联合的使用是不安全的，因为不同字段的内存表示可能不同
    // 读取与最后写入的字段类型不同的字段可能会导致未定义行为
}