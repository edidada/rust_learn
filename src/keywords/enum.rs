// enum 关键字示例

// 1. 基本枚举
enum Direction {
    North,
    South,
    East,
    West,
}

// 2. 带数据的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 3. 实现方法的枚举
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }
    
    fn unwrap(self) -> T {
        match self {
            Option::Some(value) => value,
            Option::None => panic!("Called unwrap on None"),
        }
    }
}

fn main() {
    // 1. 使用基本枚举
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Heading north"),
        Direction::South => println!("Heading south"),
        Direction::East => println!("Heading east"),
        Direction::West => println!("Heading west"),
    }
    
    // 2. 使用带数据的枚举
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write("Hello".to_string());
    
    // 3. 使用实现了方法的枚举
    let some_value = Option::Some(42);
    let none_value = Option::None;
    
    println!("some_value is_some: {}", some_value.is_some());
    println!("none_value is_some: {}", none_value.is_some());
    println!("some_value unwrap: {}", some_value.unwrap());
}