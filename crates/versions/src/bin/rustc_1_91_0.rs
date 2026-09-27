// rustc 1.91.0 演示 —— strict_* 族、carrying 算术、file_prefix、Duration::from_hours、Cell::as_array_of_cells、char_boundary
// 注意：本仓库以 rustc 1.98 运行；1.91 稳定的 API 在 1.98 均可用。
use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn main() {
    println!("rustc 1.91.0 演示");

    println!("\n1. 整数 strict_*（1.91 稳定：溢出即 panic 的算术）");
    // 旧的 wrapping_* 回绕、checked_* 返回 Option；strict_* 直接 panic，适合"溢出即 bug"场景
    println!("2u32.strict_add(3) = {}", 2u32.strict_add(3));
    println!("7u32.strict_sub(2) = {}", 7u32.strict_sub(2));
    println!("6i32.strict_mul(-2) = {}", 6i32.strict_mul(-2));
    println!("i8::MIN.strict_neg() 会 panic（-(-128) 超范围）——此处不执行");
    let r = {
        // 静默 panic 输出，演示"strict_div(0) 会 panic"这一事实
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let r = std::panic::catch_unwind(|| 7u32.strict_div(0)).is_err();
        std::panic::set_hook(default_hook);
        r
    };
    println!("7u32.strict_div(0) = panic({})", r);

    println!("\n2. uN::carrying_add / borrowing_sub（带进/借位，返回 (值, 进借位)）");
    // 连同 carry/borrow 布尔，用于多精度/大数算术；旧标准库无稳定等价
    println!("u32::MAX.carrying_add(1, false) = {:?}（值回绕 0，进位 true）", u32::MAX.carrying_add(1, false));
    println!("0u32.borrowing_sub(1, false)    = {:?}（借位 true）", 0u32.borrowing_sub(1, false));
    println!("5u32.carrying_add(2, false)     = {:?}", 5u32.carrying_add(2, false));
    println!("u32::MAX.carrying_mul_add(MAX, MAX, MAX) = {:?}", u32::MAX.carrying_mul_add(u32::MAX, u32::MAX, u32::MAX));

    println!("\n3. Path::file_prefix 与 PathBuf::add_extension / with_added_extension");
    // file_prefix：文件名去掉"最后一个 . 之后的副扩展"（archive.tar.gz 的 prefix=archive）
    let p = Path::new("archive.tar.gz");
    println!("path = {p:?}");
    println!("file_stem  = {:?}", p.file_stem());
    println!("file_prefix(新) = {:?}", p.file_prefix());
    let mut pb = PathBuf::from("/tmp/a");
    let _ = pb.add_extension("log"); // 返回 bool 表示是否成功
    println!("add_extension 后 = {pb:?}");
    println!("with_added_extension = {:?}", PathBuf::from("/tmp/a").with_added_extension("log"));

    println!("\n4. Duration::from_mins / from_hours（1.91 稳定）");
    // 旧版只有 from_secs/from_millis 等秒以下粒度；分钟/小时要手乘
    println!("Duration::from_hours(2)  = {:?}", Duration::from_hours(2));
    println!("Duration::from_mins(90)  = {:?}（即 1.5h）", Duration::from_mins(90));

    println!("\n5. C 式变参声明稳定（sysv64/win64/efiapi/aapcs）—— 记叙");
    // 1.91 起这些 ABI 也允许在 extern 块*声明* variadic 函数（不能定义）：
    //   unsafe extern "sysv64" { fn f(fmt: *const i8, ...) -> i32; }
    println!("变参声明示例（不真实调用外部）：extern \"sysv64\" {{ fn f(fmt: *const i8, ...) -> i32; }}");
    // 1.91 新 lint：integer_to_ptr_transmutes —— 用 ptr::with_exposed_provenance 替代 transmute
    let addr: usize = 0x1000;
    let _p: *const u8 = std::ptr::with_exposed_provenance(addr);
    println!("整数转指针：用 ptr::with_exposed_provenance({addr:#x}) 而不是 transmute");

    println!("\n6. Cell::as_array_of_cells（1.91 稳定）");
    // 在 &Cell<[T; N]> 上取 &[Cell<T>; N] 视图：整组可写变逐元素可写
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    let cell_of_arr = Cell::from_mut(&mut arr);
    let cells: &[Cell<i32>; 4] = cell_of_arr.as_array_of_cells();
    println!("as_array_of_cells()[2] = {}", cells[2].get());
    cells[2].set(30);
    // Cell::from_mut(&mut arr) 借走可变借用；set 后通过 Cell 读回演示
    println!("set(30) 后 cells[2] via Cell = {}", cells[2].get());
    println!("Cell 视图批量写演示完成");

    println!("\n7. str::floor_char_boundary / ceil_char_boundary（1.91 稳定）");
    let s = "你好world"; // '你'3 字节、'好'3 字节、ASCII 各 1 字节
    println!("{s:?} floor_char_boundary(1) = {}", s.floor_char_boundary(1)); // 0：'你' 起点
    println!("{s:?} floor_char_boundary(4) = {}", s.floor_char_boundary(4)); // 3：'好' 起点
    println!("{s:?} ceil_char_boundary(1)  = {}", s.ceil_char_boundary(1)); // 3：下一个边界
    println!("{s:?} ceil_char_boundary(4)  = {}", s.ceil_char_boundary(4)); // 6
}
