// rustc 1.32.0 演示 —— dbg! + Self 构造器 + 整型 bytes API
// 该版本 introduces:
// 1) dbg! 宏稳定：打印表达式值/位置到 stderr 并返回表达式值，调试利器。
// 2) Self 可作元组/单元结构体构造器和模式（此前必须写结构体名）。
// 3) 整型 to_be_bytes/from_be_bytes 族稳定（字节序明确转换）。
// 4) 模块路径解析规则：优先解析当前模块中的项。
// 5) 默认分配器从 jemalloc 改为系统分配器（println 讲解）。

struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y) // 1.32 前：必须写 Point(x, y)
    }
    fn is_origin(&self) -> bool {
        match self {
            Self(0, 0) => true, // Self 作为模式
            _ => false,
        }
    }
}

// 模块路径优先解析当前模块项的演示
enum Color {
    Red,
    Blue,
}

fn main() {
    println!("rustc 1.32.0 演示");

    // 1. dbg!（输出到 stderr，返回表达式值）
    println!("\n1. dbg! 宏");
    let a = 2;
    let b = dbg!(a * 2) + 1; // stderr: [src/bin/.._1_32_0.rs:行号] a * 2 = 4
    println!("b = {}", b);

    // 2. Self 构造器与模式
    println!("\n2. Self 构造器/模式");
    let p = Point::new(0, 0);
    println!("is_origin = {}, !=(3,4) -> {}", p.is_origin(), Point::new(3, 4).is_origin());

    // 3. 整型 bytes API
    println!("\n3. u32::to_be_bytes / from_be_bytes");
    let n: u32 = 0x0403_0201;
    let be = n.to_be_bytes();
    println!("to_be_bytes = {:?}", be); // [4, 3, 2, 1]
    println!("from_be_bytes 回 = {}", u32::from_be_bytes(be));

    // 4. 无前缀路径优先解析当前模块项
    println!("\n4. 模块路径优先解析当前模块的项");
    use Color::*; // 解析到本 crate 顶层的 Color，而非外部同名 crate
    println!("use Color::*; -> {:?}", Red as u8);

    // 5. PathBuf FromStr / Box<[T]> FromIterator
    println!("\n5. PathBuf FromStr / Box<[T]> FromIterator");
    let pb: std::path::PathBuf = "./a.txt".parse().unwrap();
    println!("\"./a.txt\".parse::<PathBuf>() = {}", pb.display());
    let boxed: Box<[i32]> = (1..=3).collect();
    println!("Box<[i32]> = {:?}", boxed);

    // 6. Self 用在类型定义
    println!("\n6. Self 在类型定义中");
    struct Wrapper(std::mem::ManuallyDrop<String>);
    let _ = std::mem::size_of::<Wrapper>(); // 此处 Self 需在 impl 内，示意即可
    println!("(where Self: ... 与 Box<Self> 均可用，见笔记)");
}
