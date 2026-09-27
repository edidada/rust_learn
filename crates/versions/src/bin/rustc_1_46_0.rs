// rustc 1.46.0 演示 —— const fn if/match/loop / const 切片强转 / #[track_caller] / Option::zip / leading_ones 等
use std::num::NonZeroU8;

// 1.46.0 起语言特性：const fn 内允许 if/match/loop（此前 const fn 只能顺序求值）。
const fn classify(n: u32) -> u32 {
    if n < 10 {
        n * 10 // if 作为表达式参与 const 求值
    } else if n < 100 {
        match n % 10 {
            // match 同样可以在 const fn 中使用
            0 => 999,
            _ => n + 1,
        }
    } else {
        let mut total = 0;
        let mut i = 0;
        // loop/while 也允许在 const fn 中（此处演示 while 循环累加）
        while i < 3 {
            total += n;
            i += 1;
        }
        total
    }
}

// 1.46.0 起另一语言特性：const fn 中可把数组强转/强制转换为切片 &[T]。
const fn sum_of(arr: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < arr.len() {
        s += arr[i];
        i += 1;
    }
    s
}

// 1.46.0 起稳定：#[track_caller] 让 panic 消息携带调用者位置。
#[track_caller]
fn assert_even(n: u32) {
    if n % 2 != 0 {
        panic!("{} 不是偶数", n); // panic 位置指向调用 assert_even 处，而非本函数内部
    }
}

// 1.46.0 起：元组字段递归索引不需要括号（.0.0 直接写）。
// 1.46.0 起：mem::forget 成为 const fn（此前不能在 const 上下文使用）。
// mem::transmute 也可用于 statics/constants（此处不演示，涉及 unsafe 裸转换不必要）。
const FORGET_ME: u32 = {
    let x = 42u32;
    std::mem::forget(x); // const fn 在 1.46.0 稳定；该 const 上下文执行 forget 不会中止求值
    7
};

fn main() {
    println!("rustc 1.46.0 演示");

    // 1.46.0：if/match/loop 进入 const fn，编译期即可条件求值。
    println!("\n1. const fn 中的 if/match/loop");
    println!("classify(5)   = {}（走 if 分支 n*10）", classify(5));
    println!("classify(50)  = {}（走 match 命中 _）", classify(50));
    println!("classify(10)  = {}（match 命中 0 → 999）", classify(10));
    println!("classify(200) = {}（走 while 累加 3 次）", classify(200));

    // 1.46.0：const fn 中数组→切片强转可用。
    println!("\n2. const fn 中强转为 &[T]");
    const ARR: [i32; 4] = [1, 2, 3, 4];
    const SUM: i32 = sum_of(&ARR); // &[i32;4] 强转为 &[i32]，全程 const 求值
    println!("sum_of(&[1,2,3,4]) 的 const 结果 = {}", SUM);
    let runtime_sum = sum_of(&[5, 6, 7]);
    println!("运行期调用同函数 = {}（同一 fn 两条求值路径）", runtime_sum);

    // 1.46.0：#[track_caller]——panic 报告调用者位置。
    println!("\n3. #[track_caller]");
    assert_even(2); // 正常通过
    println!("assert_even(2) 通过；若传入奇数，panic 位于调用行而非函数体内部。");
    // 想亲自看效果可解开下行注释，panic 位置将是本行：
    // assert_even(3);

    // 1.46.0 稳定 API：Option::zip。
    println!("\n4. Option::zip");
    let a = Some(1);
    let b = Some("x");
    let zipped = a.zip(b); // 两个 Some → Some((1, "x"))
    println!("Some(1).zip(Some(\"x\")) = {:?}", zipped);
    println!("Some(1).zip(None::<char>) = {:?}", Some(1).zip(None::<char>));
    println!("None::<i32>.zip(Some(2))  = {:?}", None::<i32>.zip(Some(2)));

    // 1.46.0 稳定库：所有整数 leading_ones/trailing_ones。
    println!("\n5. leading_ones / trailing_ones");
    let v: u8 = 0b1110_1100;
    println!("0b11101100u8.leading_ones()  = {}", v.leading_ones());
    println!("0b11101100u8.trailing_ones() = {}", v.trailing_ones());
    println!("0u32.leading_ones() = {}", 0u32.leading_ones());

    // 1.46.0 稳定库：NonZero 类型 TryFrom 可零类型；String 从 char；vec IntoIter/Drain 实现 AsRef。
    println!("\n6. NonZero TryFrom / String From<char> / 其他 trait 实现");
    // TryFrom<u8> for NonZeroU8：0 失败，非 0 成功
    println!("NonZeroU8::try_from(5) = {:?}", NonZeroU8::try_from(5u8));
    println!("NonZeroU8::try_from(0) = {:?}", NonZeroU8::try_from(0u8));
    println!("String::from('R') = {:?}", String::from('R'));
    let v = vec![10, 20];
    let iter = v.into_iter(); // 先绑定：临时值直接链 .as_ref() 会立即释放
    let slice_ref: &[i32] = iter.as_ref(); // vec::IntoIter<T>: AsRef<[T]>
    println!("vec IntoIter as_ref() = {:?}", slice_ref);
    let mut v2 = vec![1, 2, 3, 4];
    let drain = v2.drain(..2);
    println!("vec::Drain as_slice() = {:?}（已出队列部分）", drain.as_ref());
}
