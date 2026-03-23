#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,       // 无数据的单元变体
    Move,         // 无数据的单元变体
    Echo,         // 无数据的单元变体
    ChangeColor,  // 无数据的单元变体
    Quit,         // 无数据的单元变体
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
