// rustc 1.89.0 演示 —— 显式推断 const 泛型参数、repr(u128)、File 锁、leak、Result::flatten、NonNull、LazyLock
// 注意：本仓库以 rustc 1.98 运行；1.89 稳定的 API 在 1.98 均可用。
use std::cell::LazyCell;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::num::NonZero;
use std::path::PathBuf;
use std::ptr::NonNull;

// 1.89 稳定 generic_arg_infer：const 泛型参数写 `_` 交给编译器推断。
// 旧写法必须显式给出常量值；现在数组长度处可直接写 `_`。
fn const_infer<const N: usize>(arr: [u32; N]) -> usize {
    let _copy: [u32; _] = arr; // ← 新：const 参数处写 `_`，由 arr 推断
    N
}

// repr(u128) 在 1.89 稳定（repr128）：枚举判别类型可以是 128 位整数。
#[repr(u128)]
#[derive(Debug)]
enum Wide {
    Big = 0x1_0000_0000_0000_0000_0000_0000, // 2^96，超过 u64 范围
    Small = 1,
}

fn main() {
    println!("rustc 1.89.0 演示");

    println!("\n1. 显式推断 const 泛型参数（const `_`）");
    let a = [10u32, 20, 30];
    println!("const_infer([10,20,30]) = {}（`[u32; _]` 推断为 3）", const_infer(a));

    println!("\n2. #[repr(u128)] 大判别值枚举");
    // 判别值 2^96 只有 u128 装得下；1.89 之前 repr128 需要 nightly
    println!("Wide::Big = {0:?}（判别值 = 2^96）", Wide::Big);
    println!("Wide::Small = {0:?}", Wide::Small);

    println!("\n3. File::lock / try_lock / unlock（advisory 文件锁稳定）");
    let dir = std::env::temp_dir();
    let p = dir.join("rust_learn_1_89_lock.txt");
    let mut f = File::create(&p).unwrap();
    f.lock_shared().unwrap(); // 加共享锁
    println!("lock_shared 成功");
    match f.try_lock_shared() {
        Ok(()) => println!("再次 try_lock_shared 成功（共享锁可重入）"),
        Err(e) => println!("try_lock_shared 失败：{e}"),
    }
    f.unlock().unwrap();
    println!("unlock 释放成功");
    drop(f);
    let _ = std::fs::remove_file(&p);

    println!("\n4. OsString::leak / PathBuf::leak（1.89 稳定）");
    let os: OsString = OsStr::new("leaked-1.89").to_os_string();
    let leaked: &OsStr = os.leak(); // 'static 化，零拷贝
    let pb = PathBuf::from("/tmp/x/y");
    let leaked_path: &std::path::Path = pb.leak();
    println!("leaked OsStr = {leaked:?}; leaked Path = {}", leaked_path.display());

    println!("\n5. Result::flatten —— Result<Result<T,E>,E> -> Result<T,E>");
    let nested: Result<Result<i32, String>, String> = Ok(Err("inner err".into()));
    println!("flatten(Ok(Err(..))) = {:?}", nested.flatten());
    let ok: Result<Result<i32, String>, String> = Ok(Ok(7));
    println!("flatten(Ok(Ok(7))) = {:?}", ok.flatten());

    println!("\n6. NonNull::from_ref / from_mut 及 provenance 系（1.89 稳定）");
    let mut x: u32 = 42;
    let p_mut: NonNull<u32> = NonNull::from(&mut x);
    let p_ref: NonNull<u32> = NonNull::from(&x);
    unsafe { *p_mut.as_ptr() = 99; }
    println!("NonNull::from_ref/from_mut 读写 x = {}", unsafe { *p_ref.as_ptr() });
    let c = NonZero::<char>::new('a').unwrap();
    println!("NonZero<char> = {c:?}（'\\0' 不是合法 char 值，故可 NonZero）");

    println!("\n7. LazyCell/LazyLock 的可变访问（1.89 DerefMut + 后续 get/force_mut）");
    // 1.89：LazyCell/LazyLock 实现 DerefMut —— 初始化后可直接可变借用
    let mut l2: LazyCell<Vec<i32>> = LazyCell::new(|| vec![1, 2, 3]);
    l2.push(4); // DerefMut 稳定后才合法
    println!("LazyCell<Vec> deref_mut push 后 = {:?}", LazyCell::force(&l2));
    let _ = p_mut;
}
