// rustc 1.92.0 演示 —— &raw union 字段、new_zeroed 系（MaybeUninit 形态）、RwLockWriteGuard::downgrade、insert_entry、NonZero::div_ceil
// 注意：本仓库以 rustc 1.98 运行；1.92 稳定的 API 在 1.98 均可用（本工具链中 new_zeroed 返回 MaybeUninit 包装）。
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

union U {
    i: i32,
    f: f32,
}

fn main() {
    println!("rustc 1.92.0 演示");

    println!("\n1. &raw const/mut 取 union 字段（1.92 起安全代码可用）");
    let u = U { i: 0x4048_f5c3 };
    // 1.92 之前给 union 字段建裸指针要进 unsafe 块；现在 &raw 直接允许 —— 取地址本身无 UB
    let addr_i = &raw const u.i;
    let addr_f = &raw const u.f;
    println!("&raw u.i = {:p}", addr_i);
    println!("&raw u.f = {:p}（与 u.i 共享同一存储，地址相同）", addr_f);

    println!("\n2. Box::new_zeroed / new_zeroed_slice（1.92 稳定）");
    // 旧写法：unsafe { alloc zeroed + assume_init } 手工三件套；1.92 稳定化。
    // 本工具链（1.98）签名返回 Box<MaybeUninit<T>> —— 用 assume_init 转成真实值：
    let mut b: Box<MaybeUninit<[u8; 4]>> = Box::new_zeroed();
    unsafe {
        *b.as_mut_ptr() = [1, 2, 3, 4]; // 零分配上直接初始化
    }
    println!("Box::new_zeroed + assume_init = {:?}", unsafe { b.assume_init_read() });
    let bs: Box<[MaybeUninit<f64>]> = Box::new_zeroed_slice(3);
    println!("Box::new_zeroed_slice(3)（零值即合法值，可 assume_init_ref） = {:?}", unsafe {
        bs.assume_init_ref()
    });

    println!("\n3. Rc::new_zeroed / Arc::new_zeroed（1.92 稳定，同为 MaybeUninit 形态）");
    let rc: std::rc::Rc<[MaybeUninit<i32>]> = std::rc::Rc::new_zeroed_slice(4);
    println!("Rc::new_zeroed_slice(4)（零值=0 合法） = {:?}", unsafe { rc.assume_init_ref() });
    let arc: Arc<MaybeUninit<u64>> = Arc::new_zeroed();
    println!("Arc::new_zeroed（写入前） = {:?}", unsafe { arc.assume_init_ref() });

    println!("\n4. RwLockWriteGuard::downgrade（写锁降级为读锁）");
    // 旧做法：drop 写锁再 read() —— 之间可能被其他写者插队；downgrade 原子降级无间隙
    let lock = RwLock::new(42);
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        let r = RwLockWriteGuard::downgrade(w);
        println!("降级后读到 = {}", *r);
    }
    println!("最终 = {}", *lock.read().unwrap());

    println!("\n5. btree_map::Entry::insert_entry（1.92 稳定，返回 OccupiedEntry）");
    use std::collections::btree_map::Entry;
    let mut m: std::collections::BTreeMap<&str, u32> = std::collections::BTreeMap::new();
    match m.entry("k") {
        Entry::Vacant(v) => {
            let mut occ = v.insert_entry(7); // 返回 OccupiedEntry，可继续 get_mut
            *occ.get_mut() += 3;
        }
        Entry::Occupied(mut o) => {
            o.insert(99); // OccupiedEntry::insert 覆盖式插入（insert_entry 只在 Vacant/Entry 上）
        }
    }
    println!("m[\"k\"] = {:?}", m.get("k"));

    println!("\n6. NonZero<uN>::div_ceil（1.92 稳定：NonZero 上的向上取整除法）");
    // 注意：这是 NonZero 方法 —— 除数是 NonZero，返回值同样类型；u32::div_ceil(u32) 是另一回事
    let n4 = std::num::NonZeroU32::new(4).unwrap();
    println!("nz7.div_ceil(nz4) = {}", std::num::NonZeroU32::new(7).unwrap().div_ceil(n4)); // 2
    println!("nz8.div_ceil(nz4) = {}", std::num::NonZeroU32::new(8).unwrap().div_ceil(n4)); // 2
    println!("u32::div_ceil(u32)（普整数）7/4 = {}", 7u32.div_ceil(4));

    println!("\n7. 其他 1.92 变化（记叙）");
    const ROT: [u8; 4] = {
        let mut a = [1, 2, 3, 4];
        a.rotate_left(1); // rotate_left/right 进 const 上下文
        a
    };
    println!("const rotate_left(1) = {ROT:?}");
    println!("unused_must_use 不再对 Result<(), !> / ControlFlow<!, ()> 告警");
    println!("iter::Repeat::last/count 改为 panic（原先是无限循环）");
}
