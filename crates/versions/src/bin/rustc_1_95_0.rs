// rustc 1.95.0 演示 —— if let guard、MaybeUninit/Cell as_*、Atomic*::update、cold_path、*_mut 系列、Layout
// 注意：本仓库以 rustc 1.98 运行；1.95 稳定的 API 在 1.98 均可用。
use std::cell::Cell;
use std::collections::{LinkedList, VecDeque};
use std::hint::cold_path;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering};

fn main() {
    println!("rustc 1.95.0 演示");

    println!("\n1. match 臂上的 if let guard（1.95 稳定）");
    // 旧版本守卫只能写布尔表达式；if let guard（if_let_guard）1.95 起稳定
    let v: Option<Result<i32, u8>> = Some(Ok(-7));
    match v {
        Some(Ok(n)) if let 1..=10 = n.abs() => println!("合格值 = {n}"),
        Some(Ok(n)) => println!("过大 = {n}"),
        Some(Err(e)) => println!("err = {e}"),
        None => println!("none"),
    }

    println!("\n2. MaybeUninit / Cell 转换 impl（1.95 稳定）");
    let parts: [MaybeUninit<u8>; 4] =
        [MaybeUninit::new(1), MaybeUninit::new(2), MaybeUninit::new(3), MaybeUninit::new(4)];
    let whole: MaybeUninit<[u8; 4]> = parts.into(); // From<[MaybeUninit<T>; N]> 稳定
    let slice_view: &[MaybeUninit<u8>] = whole.as_ref(); // AsRef<[MaybeUninit<T>]> 稳定
    println!("MaybeUninit<[u8;4]>::as_ref 切片视图 len = {}", slice_view.len());
    let mut cells: [u32; 3] = [1, 2, 3];
    let cell_arr: &Cell<[u32; 3]> = Cell::from_mut(&mut cells);
    let sub: &[Cell<u32>; 3] = cell_arr.as_ref(); // Cell<[T;N]>: AsRef<[Cell<T>;N]> 稳定
    sub[1].set(20);
    println!("Cell<[T;N]>::as_ref → sub[1] = {}", cells[1]);
    let slice_cells: &Cell<[u32]> = Cell::from_mut(&mut cells[..]);
    let u_view: &[Cell<u32>] = slice_cells.as_ref(); // Cell<[T]>: AsRef<[Cell<T>]> 稳定
    println!("Cell<[T]> as_ref 得 [Cell<T>]，len = {}", u_view.len());

    println!("\n3. Atomic*::update / try_update（1.95 稳定）");
    // 签名：update(set_order, fetch_order, f)；旧写法要手写 load→算→CAS 循环
    let ai = AtomicI32::new(2);
    let new_val = ai.update(Ordering::SeqCst, Ordering::SeqCst, |old| old * 10);
    println!("AtomicI32::update(x*10) = {new_val}");
    let ab = AtomicBool::new(false);
    println!("AtomicBool::try_update(!b) = {:?}",
        ab.try_update(Ordering::SeqCst, Ordering::SeqCst, |old| Some(!old)));
    let mut slot = 5u64;
    let ap: AtomicPtr<u64> = AtomicPtr::new(&mut slot);
    let prev = ap.update(Ordering::SeqCst, Ordering::SeqCst, |p| p); // 恒等更新，返回旧指针
    println!("AtomicPtr::update 返回旧值 = {}", unsafe { *prev });

    println!("\n4. ptr::as_ref_unchecked（1.95 稳定）");
    let x = 3u32;
    let p: *const u32 = &x;
    // 契约：对齐且指向已初始化值时省掉 Option 判断直接给 &T
    println!("(*const as_ref_unchecked) = {}", unsafe { p.as_ref_unchecked() });

    println!("\n5. Vec/VecDeque/LinkedList 的 *_mut 系列（1.95：插入并返回 &mut）");
    let mut v: Vec<String> = Vec::new();
    v.push_mut("a".into()).push_str("-suffix"); // push_mut 返回 &mut T
    v.insert_mut(0, "z".into());
    println!("Vec          = {v:?}");
    let mut q: VecDeque<u32> = VecDeque::new();
    *q.push_front_mut(1) += 10;
    *q.push_back_mut(3) *= 5;
    q.insert_mut(1, 2);
    println!("VecDeque     = {q:?}");
    let mut ll: LinkedList<char> = LinkedList::new();
    *ll.push_front_mut('x') = 'y'; // 覆盖刚推的值（返回可变引用）
    *ll.push_back_mut('z') = 'w';
    println!("LinkedList   = {ll:?}");

    println!("\n6. hint::cold_path（1.95 稳定）");
    // 告知优化器"本路径几乎不会执行"；取代手写 unlikely 内在函数组合
    fn demo(x: u32) -> u32 {
        match x {
            1 => 10,
            2 => 20,
            _ => {
                cold_path(); // 极少走到
                u32::MAX
            }
        }
    }
    let d1 = demo(1);
    let d9 = demo(9);
    println!("cold_path demo：demo(1)={d1} demo(9)={d9}");

    println!("\n7. Layout::repeat / dangling_ptr（1.95 稳定）");
    // repeat：同布局 n 份 + 返回重对齐布局与 padding 间距
    let (lay, pad) = std::alloc::Layout::new::<u64>().repeat(3).expect("repeat 失败");
    println!("Layout::new::<u64>().repeat(3) → size={} pad={}", lay.size(), pad);
    println!("Layout::dangling_ptr：取仅含对齐信息的合法悬垂指针");

    println!("\n8. bool::TryFrom<整数>（1.95 稳定）");
    println!("bool::try_from(0i64) = {:?}", bool::try_from(0i64));
    println!("bool::try_from(3i64) = {:?}", bool::try_from(3i64)); // Err：非 0/1
}
