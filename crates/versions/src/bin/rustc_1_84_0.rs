// rustc 1.84.0 演示 —— provenance API（addr/expose_provenance）、&raw const *ptr 安全化、isqrt、dangling
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.84 稳定的 API 在 1.98 均可用。
use std::ptr;

fn main() {
    println!("rustc 1.84.0 演示");

    println!("\n1. provenance API：addr / expose_provenance / with_addr（1.84 稳定）");
    // 旧 API（as_ptr as usize + from_exposed_addr）被 provenance 语义取代：
    //   addr(): 只取地址不保留 provenance（int 不可解引用）
    //   expose_provenance(): 取地址并"暴露"provenance，之后可用 with_addr 取回
    let x = 97u32;
    let p = &raw const x;
    let raw_addr = p.addr(); // usize，不含 provenance
    println!("addr() = {raw_addr:#x}（仅地址，语义上不可解引用）");
    let exposed = p.expose_provenance();
    let restored = ptr::with_exposed_provenance::<u32>(exposed);
    println!("expose + with_exposed_provenance 读回 = {}", unsafe { *restored });
    // map_addr：以地址做运算保持 provenance（这里仅演示无偏移恒等）：
    let mapped = p.map_addr(|a| a);
    println!("map_addr(恒等) 读回 = {}", unsafe { *mapped });

    println!("\n2. &raw const *ptr —— 对裸指针解引用取新指针不再 unsafe（1.84）");
    let arr = [10, 20, 30];
    let base = arr.as_ptr();
    // 旧写法：unsafe { &raw const *base } —— "解引用再取地址"曾经是 unsafe；
    // 1.84 起：新建指针不读内存，所以 &raw const *p 是安全操作；指针运算 add 与真正的读仍要 unsafe：
    let elem1: *const i32 = &raw const *unsafe { base.add(1) }; // ← &raw const *p 本身安全
    println!("&raw const *base.add(1) 指向 = {}", unsafe { *elem1 });

    println!("\n3. isqrt（1.84 稳定）");
    println!("25u32.isqrt() = {}", 25u32.isqrt());
    println!("4096i32.isqrt() = {}", 4096i32.isqrt());
    println!("(-5i32).checked_isqrt() = {:?}", (-5i32).checked_isqrt());
    println!("NonZero::isqrt(NonZero(9)) = {:?}", std::num::NonZero::new(9u32).map(|n| n.isqrt()));

    println!("\n4. ptr::dangling / without_provenance（1.84 稳定）");
    // dangling(n)：对齐正确的"悬空但良构"指针（给分配器 API 做零容量用）；
    // without_provenance(n)：只有地址、无 provenance，解引用是 UB。
    let d = ptr::dangling::<u8>(); // 无参版：对齐自 T
    println!("dangling::<u8>().addr() = {:#x}", d.addr());
    let w = ptr::without_provenance::<u8>(0x1000);
    println!("without_provenance(0x1000).addr() = {:#x}（不可解引用）", w.addr());
    println!("is_null() = {}（dangling 不是 null）", d.is_null());

    println!("\n5. Pin::as_deref_mut（1.84 稳定）");
    use std::pin::Pin;
    // 1.84 稳定 Pin::as_deref_mut —— 形态：Pin<&mut Pin<Ptr>> → Pin<&mut Ptr::Target>。
    // 先 as_mut 从 Pin<Box<Pin<String>>> 得 Pin<&mut Pin<String>>，再 as_deref_mut 穿过内层：
    let mut nested: Pin<Box<Pin<String>>> = Box::pin(Pin::new(String::from("pin-84")));
    let inner: Pin<&mut str> = nested.as_mut().as_deref_mut();
    println!("as_deref_mut → inner 内容 = {:?}", &*inner);
    println!("len = {}", inner.len());
}
