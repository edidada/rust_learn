#[derive(Debug)]
#[allow(dead_code)] // 字段仅经 derive(Debug) 输出，显式读取场景留给练习
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
#[allow(dead_code)] // 各变体字段仅经 derive(Debug) 输出，显式读取场景留给练习
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u32, height: u32 },       // 结构体变体，有命名字段
    Move(Point),                              // 元组变体，包含 Point 结构体
    Echo(String),                             // 元组变体，包含 String
    ChangeColor(u8, u8, u8),                  // 元组变体，包含三个 u8
    Quit,                                     // 单元变体，不包含数据
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
