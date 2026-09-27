// rustc 1.94.0 演示 —— array_windows、element_offset、LazyLock/LazyCell::get、TryFrom<char>、Peekable::next_if_map
// 注意：本仓库以 rustc 1.98 运行；1.94 稳定的 API 在 1.98 均可用。
use std::cell::LazyCell;
use std::iter::Peekable;
use std::sync::LazyLock;

fn main() {
    println!("rustc 1.94.0 演示");

    println!("\n1. <[T]>::array_windows（1.94 稳定：定长滑动窗口）");
    // 旧写法：windows(n).map(转 [T; N]) 手工转；array_windows 直接迭代 &[u8; N]
    let data = [1u8, 2, 3, 4, 5];
    for (i, w) in data.array_windows::<3>().enumerate() {
        println!("window {i} = {w:?}");
    }

    println!("\n2. <[T]>::element_offset（1.94 稳定：元素引用 → 下标）");
    let doubles = [1u64, 2, 3, 4];
    // 签名：element_offset(&self, element: &T) -> Option<usize>（按指针位置，非值比较）
    let num = &doubles[2];
    println!("element_offset(&doubles[2]) = {:?}", doubles.element_offset(num)); // Some(2)

    println!("\n3. LazyLock::get / get_mut / force_mut（1.94 稳定，关联函数形态）");
    static L: LazyLock<u32> = LazyLock::new(|| 7 * 6);
    // 签名：get(this: &LazyLock) -> Option<&T>；get/force_mut 需 &mut 且 UnwindSafe 语义
    println!("LazyLock::get(&L) = {:?}", LazyLock::<u32>::get(&L));

    println!("\n4. LazyCell::get / get_mut（1.94 稳定，关联函数形态，线程内版本）");
    let mut cell: LazyCell<Vec<i32>> = LazyCell::new(|| vec![1, 2]);
    println!("LazyCell::get(&cell) = {:?}", LazyCell::<Vec<i32>>::get(&cell));
    if let Some(v) = LazyCell::<Vec<i32>>::get_mut(&mut cell) {
        v.push(3);
    }
    println!("get_mut push 后 = {:?}", LazyCell::<Vec<i32>>::get(&cell));

    println!("\n5. impl TryFrom<char> for usize（1.94）");
    // char 的码点值可靠地转成 usize
    let c = '中'; // U+4E2D
    println!("usize::try_from('中') = {:?}", usize::try_from(c)); // 20010
    println!("usize::try_from('a') = {:?}", usize::try_from('a')); // 97

    println!("\n6. Peekable::next_if_map（1.94 稳定：条件消费并变换）");
    // 签名：next_if_map(f: FnOnce(Item) -> Result<R, Item>) -> Option<R> —— Err 时把元素放回队首
    // slice iter 的 Item = &u8，所以闭包拿引用；Err 也返回同款引用才能"放回"
    let mut it: Peekable<std::slice::Iter<u8>> = [1u8, 5, 9].iter().peekable();
    let r1 = it.next_if_map(|it| if *it > 4 { Ok(*it) } else { Err(it) });
    println!("next_if_map(x>4 则取) = {:?}", r1); // None（1 <= 4，退回）
    let r2 = it.next_if_map(|it| if *it > 0 { Ok(*it) } else { Err(it) });
    println!("next_if_map(x>0 则取) = {:?}", r2); // Some(5)
    println!("剩余 next = {:?}", it.next());

    println!("\n7. 新数学常量与 mul_add const 化");
    const G: f64 = std::f64::consts::EULER_GAMMA; // 欧拉-马歇罗尼常数 γ（1.94 稳定）
    const PHI: f64 = std::f64::consts::GOLDEN_RATIO; // 黄金比例 φ（1.94 稳定）
    const MAD: f64 = 1.5f64.mul_add(2.0, 1.0); // mul_add 进 const（1.94）
    println!("EULER_GAMMA  = {G}");
    println!("GOLDEN_RATIO = {PHI}");
    println!("const 1.5*2+1 = {MAD}");

    println!("\n8. 兼容性：std 宏改由 prelude 导入（记叙）");
    println!("matches!/panic! 等改经 prelude；自定义同名宏 + glob import 将歧义报错，需显式 use");
}
