// match 关键字示例

// 1. 基本模式匹配
enum Direction {
    North,
    South,
    East,
    West,
}

// 2. 带数据的模式匹配
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 1. 基本模式匹配
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Heading north"),
        Direction::South => println!("Heading south"),
        Direction::East => println!("Heading east"),
        Direction::West => println!("Heading west"),
    }
    
    // 2. 带数据的模式匹配
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
    
    // 3. 匹配数字范围
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3..=5 => println!("Three to five"),
        _ => println!("Other"),
    }
    
    // 4. 匹配Option
    let some_value = Some(42);
    match some_value {
        Some(x) => println!("Some value: {}", x),
        None => println!("No value"),
    }
}